# Double entry bookkeeping

_This doesn't actually perform any check that the balances balance out, nor does
it support transactions that target more than one account._

This is largely based on the following article:
[Double entry accounting for engineers — Anvil](https://anvil.works/blog/double-entry-accounting-for-engineers).

<!--toc:start-->

- [Double entry bookkeeping](#double-entry-bookkeeping)
  - [Running](#running)
  <!--toc:end-->

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

The output looks like

```
-- Balance sheets -------------------------------
| Cash in Bank                   | €  -40176.96 |
| Fixed Assets                   | €   28774.95 |
-------------------------------------------------

-- Profit/loss ----------------------------------
| Sales                          | €   11402.01 | <-- perhaps not entirely intuitively, this is the balance after all transactions are processed
| \ Office supplies              | €   -4280.87 |
| \ Store sale                   | €   -1377.38 |
| \ Equipment maintenance        | €    1748.88 |
| \ Client payment               | €    2630.07 |
| \ Service revenue              | €   -4972.06 |
-------------- snip -----------------------------
```
