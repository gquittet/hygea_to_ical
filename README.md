# Hygea to iCal

[![CI](https://github.com/gquittet/hygea_to_ical/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/gquittet/hygea_to_ical/actions/workflows/ci.yml)


## Discontinued ⚠️

New project: https://github.com/gquittet/recycle-cli

The old Hygea API has been migrated to the Recycle one since 2022.

So this application will not work after the 31st December 2021.

## Goal

Hygea provides a calendar via PDF and an application called
[Recycle](https://www.calendrierdecollectes.be/).

I just wanted to use an iCal file to import it in my calendar. So I made this
little executable that call their API and generate an `hygea.ics` that you can
import to your GMail, NextCloud or Apple calendar.

*Hygea is an intermunicipal company born from the merger of the Public Waste
Management Sector of IDEA, in charge of waste collection, and the intermunicipal
company ITRADEC, in charge of waste treatment for 24 municipalities spread over
the Mons-Borinage-Centre region.*

## How to use

```bash
./hygea_to_ical POSTAL_CODE <STREET_NAME (optional)>
```

where `POSTAL_CODE` is a postal code managed by Hygea in Belgium.

### Help

```bash
./hygea_to_ical --help
```


### Example

```bash
./hygea_to_ical 7340
```

```bash
./hygea_to_ical 7000 "rue de nimy"
```
