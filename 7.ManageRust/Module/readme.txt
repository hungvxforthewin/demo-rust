Defining Modules to Control Scope and Privacy
     the use keyword that brings a path into scope
     the pub keyword to make items public
1. Modules Quick Reference
Here’s how modules, paths, the use keyword, and the pub keyword work in the compiler, and how most developers organize their code
Rule để thực hiện chúng
    + Start from the crate root: usually src/lib.rs for a library crate or src/main.rs for a binary crate
    + Declaring modules: In the crate root file, you can declare a new module named, say, “garden”, with mod garden

            In the file src/garden.rs
            In the file src/garden/mod.rs

