fn main() {
    
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello, world!";
            color: green;
        }
    }
}