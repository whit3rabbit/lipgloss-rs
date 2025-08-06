# Tree-in-List Spacing Issue

## Problem Description

When a `lipgloss_tree::Tree` is embedded within a `lipgloss_list::List` (via `item_node()`), 
the tree symbols get an extra space after them, causing misalignment with golden test files.

## Current Behavior vs Expected

### Expected (from golden files):
```
├── another     (1 space after tree symbol)
│   multine     (3 spaces after │)
│   string      (3 spaces after │)
```

### Current Output:
```
├──  another    (2 spaces after tree symbol - WRONG)
│    multine    (4 spaces after │ - WRONG)
│    string     (4 spaces after │ - WRONG)
```

## Evolution of Understanding Through Testing

### Phase 1: Initial Discovery
- **Issue**: 4 list tests failing: `golden_sublist`, `golden_sublist_items`, `golden_sublist_items2`, `golden_complex_sublist`
- **Root Cause**: `list_indenter()` was returning 1 space, but golden files expected 2-space sublist indentation
- **Solution**: Changed `list_indenter()` from `" "` to `"  "` (1→2 spaces)
- **Result**: Fixed 3/4 tests, but broke `golden_complex_sublist` with tree spacing issue

### Phase 2: Architecture Analysis  
- **Discovery**: The issue is architectural, not just a spacing parameter
- **Key Insight**: Trees have their own indentation logic (`default_indenter`) but inherit parent's indenter when nested
- **Problem**: Trees nested in lists inherit `list_indenter` (2 spaces) but expect `default_indenter` (4-char patterns)
- **Test Status**: 18/19 tests passing, only complex tree case failing

### Phase 3: Smart Indentation Logic
- **Approach**: Implemented detection logic in `lipgloss-tree/src/renderer.rs` around line 600
- **Logic**: Check if parent uses `list_indenter` (2 spaces) and child uses `default_indenter` (contains │ or 4+ chars)
- **Implementation**: Added smart child_prefix calculation to avoid double-indentation
- **Result**: List indentation preserved, but tree still gets extra space internally

### Phase 4: Renderer Isolation
- **Approach**: Create fresh renderer for tree children to avoid inheriting list behavior
- **Implementation**: Added `child_uses_tree_indenter` detection in renderer child logic
- **Result**: Tree prefix calculation works correctly, but enumerator styling still produces extra space

## Current Status (After All Testing)

### Test Results
- ✅ **18/19 tests passing** - All basic list functionality works correctly
- ❌ **1/19 tests failing** - `golden_complex_sublist` has tree spacing issue only
- ✅ **Architectural fix working** - Trees no longer inherit list indentation for prefix calculation
- ❌ **Enumerator spacing issue** - Tree symbols still get exactly 1 extra space

### Working Solutions Implemented
1. **List indentation fixed**: 2-space `list_indenter()` provides correct sublist visual hierarchy
2. **Smart prefix logic**: Trees nested in lists get proper prefix without double-indentation
3. **Renderer isolation**: Trees use fresh renderer to avoid inheriting list spacing behavior

### Remaining Issue
The tree enumerator symbols (`├──`, `│`) are getting exactly 1 extra space internally, even with:
- Fresh renderer preventing list behavior inheritance
- Explicit `padding_right(1)` styling  
- Correct prefix calculation

## Technical Analysis (Updated)

### Root Cause Breakdown
1. **List Configuration**: `list_indenter()` returns `"  "` (2 spaces) ✅ WORKING - fixes sublist indentation
2. **Tree Configuration**: Tree has `padding_right(1)` on enumerator_style ❌ BROKEN - produces 2 spaces instead of 1
3. **Architecture**: Smart logic prevents inheritance conflicts ✅ WORKING - proper prefix calculation

### Key Code Locations
- **List indenter**: `lipgloss-list/src/lib.rs:328` - returns 2 spaces for sublist indentation
- **Tree padding**: Test sets `.padding_right(1)` on enumerator_style in `golden_tests.rs:212`
- **Tree renderer**: `lipgloss-tree/src/renderer.rs:494-518` - applies base style vs function style
- **Final assembly**: `lipgloss-tree/src/renderer.rs:586` - joins multiline_prefix + node_prefix + item
- **Smart logic**: `lipgloss-tree/src/renderer.rs:600-650` - prevents double-indentation

### Debug Findings Through Testing
- **Prefix calculation**: Working correctly (DEBUG showed parent=list, child=tree, avoided double-indent)
- **Tree symbol rendering**: Getting 2 spaces after `├──` instead of expected 1 space
- **Multiline indentation**: Getting 4 spaces after `│` instead of expected 3 spaces
- **All other spacing**: Working perfectly (sublists, nested lists, regular trees)

### Enumerator Style Logic (Specific Issue Area)
In `lipgloss-tree/src/renderer.rs:494-518`:
```rust
if let Some(base) = &enum_base {
    // Base style set via .enumerator_style() - use ONLY this style
    // Example: Style::new().foreground(color).padding_right(1)
    node_prefix = base.render(&node_prefix);  // ← This produces 2 spaces somehow
} else {
    // Function style logic...
}
```

**Current Hypothesis**: The `base.render(&node_prefix)` call is somehow producing an extra space beyond the intended `padding_right(1)`. This could be:
- Style rendering bug in lipgloss core
- Interaction between color formatting and padding
- Context-dependent padding behavior

### Tested Solutions That Didn't Work
1. **Function pointer comparison**: Rust doesn't allow direct function pointer comparison for indenters
2. **Reverting to 1-space list_indenter**: Breaks all sublist indentation (needs 2 spaces)  
3. **Removing child prefix logic**: Restores double-indentation problem
4. **Different renderer inheritance**: Still produces extra space in enumerator rendering

### Tested Solutions That Partially Worked
1. **Smart indentation detection**: ✅ Correctly identifies tree vs list indenters
2. **Fresh renderer for trees**: ✅ Prevents list behavior inheritance
3. **Proper prefix calculation**: ✅ Trees get correct nesting prefix

### Original Technical Analysis

#### Root Cause (Before Testing)
1. **List Configuration**: `list_indenter()` returns `"  "` (2 spaces) to fix sublist indentation
2. **Tree Configuration**: Tree has `padding_right(1)` on enumerator_style for 1 space after symbols
3. **Conflict**: When tree is nested in list, both spacings are applied somehow

### Code Locations
- List indenter: `lipgloss-list/src/lib.rs:328` - returns 2 spaces
- Tree padding: Test sets `.padding_right(1)` on enumerator_style
- Tree renderer: `lipgloss-tree/src/renderer.rs:497` - applies base style
- Final assembly: `lipgloss-tree/src/renderer.rs:579` - joins prefix, symbol, content

### Rendering Flow
1. Tree is added to list via `item_node(Box::new(tree))`
2. List's tree renderer processes the tree node
3. Tree inherits list's `list_indenter` (2 spaces)
4. Tree symbols get `padding_right(1)` from enumerator_style
5. Extra space appears between symbol and content

### Why Go Works Correctly
- Go's `list.New()` sets indenter to single space: `return " "`
- Go achieves 2-space sublist indentation through different mechanism
- Go's tree rendering handles nested context differently

## Affected Tests
- `golden_complex_sublist` - FAILS due to tree spacing
- All other list tests - PASS with current 2-space list_indenter

## Potential Solutions

### Option 1: Dynamic Indenter
Detect tree nodes and return 1 space for them, 2 spaces for others.
**Problem**: Function signature doesn't provide node type information.

### Option 2: Tree Override
Have trees set their own indenter when nested in lists.
**Problem**: Requires detecting nested context, complex.

### Option 3: Renderer Adjustment
Modify tree renderer to handle 2-space list context.
**Problem**: May break standalone tree rendering.

### Option 4: Test Adjustment
Remove padding_right(1) from test tree.
**Problem**: Diverges from Go implementation.

## Debugging Steps for Future

1. **Check spacing source**:
   ```bash
   cargo test -p lipgloss-list golden_complex_sublist -- --nocapture 2>&1 | grep "├──.*another"
   ```

2. **Verify indenter values**:
   Add debug prints in `list_indenter()` and tree renderer

3. **Test with different padding**:
   Temporarily modify tree test's `padding_right()` value

4. **Compare with Go**:
   Run Go test and examine exact byte output with `xxd`

## Status: UNRESOLVED
The issue remains in `golden_complex_sublist` test. All other functionality works correctly.