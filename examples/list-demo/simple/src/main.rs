use lipgloss_list::{roman, List};

fn main() {
    let l = List::new()
        .item("A")
        .item("B")
        .item("C")
        .item_list(List::new().items(vec!["D", "E", "F"]).enumerator(roman))
        .item("G");

    println!("{}", l);
}
