# Capstone Homework
---
## Running the Code 

```
    $ cargo build
    $ cargo run
```

## Testing the Code
```
    $ cargo test
```

---
**Task**: Write code to calculate the total charge of a customers checkout order.
> Description: Create a production code function to calculate a customers checkout charge including the sales tax, which will depend on state. there should be a single public function to take a Items (which should be a list of records) and State but you can have private helper functions/methods.
**Constraints**
> States will be limited to New Jersey, Delaware and Pennsylvania
> Items sold will be limited to "Wic Eligible", "Clothing" and "Everyting Else"
> Constraints;
> DE: no sales tax
> NJ: Sales tax rate is 6.6%
> PA: Sales tax rate is 6% (we'll ignore the local tax)
> WIC Eligible food is tax exempt in all three states
> Clothing is exempt from in PA, and NJ except Fur clothing is taxable normally in NJ, we'll call it
> taxable if the string fur appears in a clothing item name
> Everything else is covered under the default tax rates


**Testing**: 
> Write automated tests for your functions .
> Happy Path tests, items from all 3 categories and all 3 states.
> Bad data test, Including Null/None, Incorrect Data Types, negative prices.
> Edge Case tests, Including Empty Lists, Maxint vlues, fur vs non fur clothes.
