# SEC Lib

Rust library for accessing SEC public data APIs.

## What is XBRL?
The e**X**tensible**B**usiness**R**ules**L**anguage is a scheme for labelling data in financial documents with common machine-readable labels.

A **concept** is a label applied to a specific piece of data, like "AccountsPayable" or "AdvertisingExpense". Most concepts come from the US-GAAP (Generally Accepted Accounting Principles) taxonomy.

An XBRL **fact** is a a 3-tuple of (concept, time, value).

## Interesting Concepts
The GAAP taxonomy is large, and companies are unfortunately permitted to create their own custom concepts. This is a curated list of concepts which should be common amongst large filers, and show big-picture information about a company. These should be useful when comparing data across companies.

[MFAANG_Concepts.json](resources/MFAANG_Concepts.json) contains a union of concepts reported by Microsoft, Facebook, Amazon, Apple and Google (Alphabet).

> AccountsPayable 
>
> AccountsPayableCurrent
> 
> AccountsReceivableNetCurrent
> 
> 

> EntityCommonStockSharesOutstanding: Number of shares which exist.

> EntityPublicFloat: Market value of voting and non-voting shares, computed from average of bid-ask spread.

> "EarningsPerShareBasic" "Earnings Per Share, Basic"
"The amount of net income (loss) for the period per each share of common stock or unit outstanding during the reporting period."
> (net income - preferred dividends) / outstanding shares




