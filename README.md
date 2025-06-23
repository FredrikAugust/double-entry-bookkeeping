# Double entry bookkeeping

<!--toc:start-->

- [Double entry bookkeeping](#double-entry-bookkeeping)
<!--toc:end-->

This is largely based on the following article:
[Double entry accounting for engineers — Anvil](https://anvil.works/blog/double-entry-accounting-for-engineers).

It uses the file `./transactions.csv` to read the transactions and then
calculates the balance sheets for each account, and profit/loss for non-asset accounts.

The format of the CSV file is as following:

| datetime iso8601    | from account | to account   | amount  | transaction name |
| ------------------- | ------------ | ------------ | ------- | ---------------- |
| 2025-05-23T16:30:00 | Fixed Assets | Cash in Bank | 1519.38 | Online sale      |

## Running

```bash
cargo run
```
