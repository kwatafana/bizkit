---
title: BizKit Handbook
subtitle: Operating System for SMEs
author: Kwatafana Systems
date: last update 30-01-2024
...
---

# BizKit

> Offline first operating system for SMEs

## PROBLEM STATEMENT:

Like it or not, the internet is now an integral part of society.
This is not surprising, it gives us magical abilities and
connects everyone (and things) in “real time”.

Small and Mid-size Enterprises (SMEs) today can tap into the
magic of the internet, giving their business new abilities that
make them stand out to customers and enhance their day-to-day
operations.

__This is why we created BizKit – A suite of digital business
tools for modern Small and medium-sized enterprises.__

## GOALS:

- Have a suite of software that enables an SME to enhance its day-
to day operations.
- Form a marketplace from the SMEs that use __bizkit__.
- Make traditional ways of running an SME obsolete.
- Allow any individual to create and run a business without any
any censorship or other kinds of restrictions.
- Freedom in commerce.
- Have new revolutionary ideas and tools to operate an SME.

## BizKit POS

- Inventory: A POS system should also serve as good inventory
management software and make the process effortless as it is
the most essential POS feature in a system. You should be able
to monitor all the critical information about the inventory
counts, from individual SKUs to the overall number of units.
Moreover, it allows you to count the stock digitally rather
than manually, saving time and money.

- Sales: A POS system can record a simple sale and deducts the
quantity sold from the inventory. Allows you to print the
invoice (if you have a receipt printer) or email it directly to
the customer.
0 Orders: Allows you to record orders taken in
store. In other words, products that your customers would like
to buy from you but you don’t have in stock, or that you don’t
normally sell in your store. When the product becomes available
in your inventory, you will have all the necessary information
to quickly inform your customers the product is available. If
your employees receive many requests for an item that you do
not have in your inventory, offering the product can represent
additional revenues.
- Repairs: Allows you to closely monitor your customer’s items
that you repair in your shop. You will be able to repair items
such as watches or jewellery for example, and track a variety
of custom repair statuses very closely.
- Rentals: Allows you to keep track of the products that have
been rented in your shop.
0 Supplier purchases: This allows you to have control over
purchases and make sure you have the items ordered from the
suppliers.
- Employee management: Most POS systems come with some form of
employee management features, like tracking employee sales,
activity and performance. This makes it easy to see which
employees are your top earners, and which ones might require
more training. You can use these tools to run competitions and
reward your top performers.
- Appointment booking: Although it is not as common a feature,
some POS systems have appointment-booking capabilities. This
tool enables appointment booking through your online store or
website. Then, you can manage appointments and sales within one
platform. This is especially useful for those operating in
professional service industries (e.g., nail salons, wellness
spas, repairs and health).
- Reporting: Every POS system generates reports, but since the
depth of reporting varies, you’ll want one with the right
analytics for your business. Will you need basic reports for
sales per hour, inventory management or other metrics?
Look for a system with customizable reports that allow you to
filter data by date range and other factors. You may also want
a system that allows you to customize how the data is presented
and schedule automatic reports to be emailed to you.
- Dashboards: POS systems have easy-to-use dashboards that give
you a quick summary of how your business is performing. You can
often see things like sales, inventory and performance at a
glance. 


### PoS HARDWARE

- Android Tablet
- Tablet stand
- Credit card reader
- Receipt printer
- Barcode scanners
- Cash Drawer (Optional)

## Account System

In __bizkit__ there are two types of accounts:

- __`Staff`__: account for a staff/employee member.
- __`Customer`__: account for a customer/client.

### Staff Account Schema (Version 0)

| Field             | Description                                       | Data Type          |
|-------------------|---------------------------------------------------|--------------------|
| __firstname__     | First name of the staff                           | `String`           |
| __middlenames__   | Middle names of the staff                         | `Option<String>`   |
| __lastname__      | Last name of the staff                            | `String`           |
| __username__      | Username of the staff                             | `String`           |
| __email__         | Email address of the staff                        | `String`           |
| __phone__         | Phone number to contact staff                     | `Option<String>`   |
| __bio__           | Short bio of staff                                | `Option<String>`   |
| __staffid__       | Unique staff id                                   | `Option<String>`   |
| __isadmin__       | Indicates if staff is admin or not                | `bool`             |
| __groups__        | Indicates the groups the staff member belongs too | `Vec<String>`      |
| __position__      | Staff's job position                              | `String`           |
| __joined__        | Date when staff started working in company        | `DateTime`         |
| __status__        | Status of staff                                   | `StaffStatus`      |
| __lastlogin__     | Date when staff last logged in                    | `Option<DateTime>` |
| __gender__        | Gender of staff member                            | `Gender`           |
| __version__       | Data type schema version                          | `u16`              |
| __password_hash__ | Hash of password                                  | `String`           |


`StaffStatus` is the __enum__:

``` rust
enum StaffStatus{
    /// Active staff member
    Active,
    /// Retired staff member
    Restired,
    /// Resigned staff member
    Resigned,
    /// Absent staff member
    Absent,
    /// A staff memeber on holiday
    Holiday,
    /// A staff memeber on leave
    Leave,
    /// A staff memeber on vacation
    Vacation
}
```

`Gender`is the __enum__:

``` rust
enum Gender{
    Male,
    Female,
    Other
}
```

## Milestones

### Alpha:
- [ ] Inventory
- [ ] Business Plan
- [ ] Docs
- [ ] Business Profile
- [ ] Marketing Assets
- [ ] Email
- [ ] Website
- [ ] SEO
- [ ] Basic Cybersec (HTTPS)
- [ ] BizKit Cashbook

#### Alpha::Inventory
- [ ] Add product (Buy)
- [ ] Remove product (Sell)
- [ ] Delete product
- [ ] Barcode
- [ ] Basic Report
- [ ] Product tags (categories)
- [ ] cloud

#### Alpha::BizPlan
- [ ] Business Plan from template
- [ ] Export PDF
- [ ] Cloud

#### Alpha::Docs
- [ ] Cloud backup
- [ ] add doc
- [ ] remove doc

#### Alpha::Business Profile / Accounts
- [ ] Profile form
- [ ] Edit profile
- [ ] cloud backup

#### Alpha::Marketing Assets
- [ ] Flyer templates
- [ ] Logos
- [ ] Letter heads
- [ ] Share assets via social media

#### Alpha::Email
- [ ] Read Emails
- [ ] Send emails

#### Alpha::Website
- [ ] Basic htoml based pages
- [ ] page editor
- [ ] Basic page widgets

#### Alpha::SEO
- [ ] Setup the core components of the SEO engine

#### Alpha::Cybersec
- [ ] HTTPS
- [ ] Kong

#### Alpha::Cashbook
- [ ] Setup the core components of the cashbook engine. (Cashbook +
accounting).

## SIMILAR PROJECTS:

- xero: https://www.xero.com/us/                  (accounting)
- shopify: https://www.shopify.com/               (ecommerce)
- trello: https://trello.com/en                   (projects)
- toggl: https://toggl.com/                       (time)
- Paypal + Stripe: https://www.paypal.com/us/home (payments)
- Hunter: https://hunter.io/                      (emails)
- Slack: https://slack.com/                       (chat)
- G-suite: https://workspace.google.com/          (docs)
- Gusto: https://gusto.com/                       (payroll)
- Sage: https://www.sage.com                      (business)
- Avast: https://www.avast.com                    (cybersec)
- Hubspot: https://www.hubspot.com                (crm)
- Bitrix24: https://www.bitrix24.com/             (business)
