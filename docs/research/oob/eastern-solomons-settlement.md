# Eastern Solomons: settlement sheet for scenario 5

Status: **proposed.** Written by Claude on 2026-11-11 for Amit to approve. Nothing here is settled in the data. No standing has been set on the researcher's judgement: a row is `preferred` today only by the format's own rule (one row at medium grade or better with nothing against it, or two sources that agree), and everything in dispute is `disputed`.

**What scenario 5 needs.** The carrier duel of 24 to 25 August 1942: the ships of both carrier forces, the aircraft that flew, how many aircraft each side had, and where the forces were. This sheet covers, in the order the architect asked for them: what was found and how firm it is, what is disputed and my recommendation, and what was not found.

**How to read it.** `id` is a row number in the file named at the head of each group. Grade is confidence: `h`, `m` or `l`, with `/i` where the row is an inference. "Sure" is **Fair**, **Coin-toss** or **Weak**. **Bulk** means the candidates lie within 3 per cent of each other, so the choice barely moves a game figure and the items can be approved together.

**What a tick on this sheet means.** For each item, the recommended value becomes the one the derivation tool reads. Where the recommended row rests on a lone weak source it cannot be marked `preferred` under decision 0015; it stays `unconfirmed` and needs a written exception in the tool's rules file, as at Savo Island.

## 0. Three things to decide before the rows

These are choices about kinds of figure. Each one settles many rows below at once, so the recommendations in section 1 assume my answer to each.

| # | Question | Options | Recommend | Why |
| --- | --- | --- | --- | --- |
| k1 | Endurance and fuel for US ships | (a) The Navy's 1945 war-service fuel tables: a primary source, measured in service, but dated 1943 to 1945. (b) The design figure that websites repeat | (a), read at 15 knots with a normal full load | The same choice you made for Savo Island. It is the only primary source in the group |
| k2 | Maximum speed for US ships | (a) Design speed. (b) The fuel tables' estimate at war-service weight, 2 to 4 knots lower | (a) | As at Savo Island. Japanese ships have only design or trial speeds, so (b) would handicap one side |
| k3 | Aircraft capacity of a carrier | (a) The designed capacity. (b) What was embarked on 24 August | (a) as the class fact; (b) comes from the air group strengths in section 5 | The two answer different questions. The rule should fill the hangar from section 5, not from the design figure |

**A correction to a Savo Island figure is offered, not made.** Reading the Navy's fuel tables again turned up a second table: radius with a normal "full load" of fuel, where the Savo rows used the table for tanks 95 per cent full. For three classes you settled on 2026-11-10, the full-load figures differ. They are in the data as `alternative` and your settled figures stand unless you say otherwise:

| Class | Settled for Savo | The full-load table | Row |
| --- | --- | --- | --- |
| New Orleans | 7,620 at 15 knots | 6,500 at 15 knots | 834 |
| Bagley | 6,940 at 12 knots | 6,080 at 15 knots | 862 |
| Benham | 5,390 at 12 knots | 4,860 at 15 knots | 869 |

If k1 is approved as recommended, the consistent thing is to move these three to the full-load table at 15 knots as well. **Recommend: yes. Sure: fair.**

## 1. Ship classes (`ship_class_facts.csv`)

Seventy-four open items. The recommendation in each row follows one stated policy, so that it can be checked:

- **Endurance and fuel, US classes:** the Navy's fuel tables (k1). Where the tables give two fuel conditions, the normal full load.
- **Speed:** the design figure most sources give (k2). The fuel tables' estimate is listed and not chosen.
- **Everything else:** the value given by the most sources that are not Wikipedia; on a tie, the better-graded source. For Japanese ships the order of trust is the Kojinsha volume and the US technical mission, then Navypedia and combinedfleet, then the Nishida pages, then Wikipedia.

Cruise speed goes with the endurance row chosen and is not listed separately. Where a row says "at 10 kn" or "at 18 kn" the figure is not comparable with one at 15, and I say so below the tables.

### Carriers

**us-lexington-class** (Saratoga)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 40000: 403 (uboat.net, m), 885 (Navypedia, m)<br>36000: 404 (Wikipedia, l), 879 (NavSource, m)<br>37681: 884 (Navypedia, m) | 40000 | Fair | 11% |
| Speed | 33.25: 410 (Wikipedia, l), 880 (NavSource, m)<br>33: 409 (uboat.net, m)<br>31.2: 829 (Navy fuel tables, h; estimate at war-service weight)<br>33.2: 886 (Navypedia, m) | 33.25 | Bulk | 1% |
| Endurance | 10000: 405 (Wikipedia, l; at 10 kn), 881 (NavSource, m; at 10 kn)<br>8210: 823 (Navy fuel tables, h; full load)<br>9490: 825 (Navy fuel tables, h; tanks 95% full, at 15 kn)<br>10500: 887 (Navypedia, m) | 8210 | Fair | 28% |
| Fuel | 9748: 828 (Navy fuel tables, h; tanks 95% full), 889 (Navypedia, m)<br>6688: 407 (Wikipedia, l)<br>8400: 827 (Navy fuel tables, h; full load) | 9748 | Fair | 46% |
| Aircraft capacity | 90: 395 (uboat.net, m), 396 (Wikipedia, l), 883 (NavSource, m)<br>63: 890 (Navypedia, l) | 90 | Fair | 43% |

**us-yorktown-class** (Enterprise)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 19800: 560 (uboat.net, m), 561 (Wikipedia, l), 891 (cv6.org, m)<br>19875: 898 (Navypedia, m) | 19800 | Bulk | 0% |
| Speed | 32.5: 566 (uboat.net, m), 567 (Wikipedia, l), 892 (cv6.org, m), 899 (Navypedia, m)<br>33.65: 893 (cv6.org, m) | 32.5 | Fair | 4% |
| Endurance | 12000: 562 (Wikipedia, l; at 15 kn), 900 (Navypedia, m; at 15 kn)<br>10400: 894 (cv6.org, m; at 15 kn) | 12000 | Coin-toss | 15% |
| Fuel | 4280: 564 (Wikipedia, l)<br>6500: 896 (cv6.org, m)<br>4360: 902 (Navypedia, m) | 6500 | Coin-toss | 52% |
| Aircraft capacity | 90: 551 (uboat.net, m), 552 (Wikipedia, l)<br>96: 897 (cv6.org, m), 903 (Navypedia, m) | 96 | Fair | 7% |

**jp-shokaku-class** (Shokaku, Zuikaku)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 25675: 732 (Navypedia, m), 733 (Kojinsha, m)<br>25270: 188 (Nishida, m) | 25675 | Bulk | 2% |
| Speed | 34.2: 191 (Nishida, m), 740 (Navypedia, m)<br>34: 738 (Kojinsha, m)<br>34.4: 739 (Kojinsha, m)<br>34.5: 741 (Wikipedia, l) | 34.2 | Bulk | 1% |
| Fuel | 4035: 189 (Nishida, m)<br>4100: 747 (Navypedia, m)<br>4900: 748 (Wikipedia, l) | 4100 | Coin-toss | 21% |

**jp-ryujo-class** (Ryujo)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 10600: 752 (Navypedia, m), 753 (combinedfleet, m)<br>10433: 153 (Nishida, m)<br>10400: 754 (Wikipedia, l) | 10600 | Bulk | 2% |
| Speed | 29: 156 (Nishida, m), 757 (Navypedia, m)<br>29.5: 758 (Wikipedia, l) | 29 | Bulk | 2% |
| Fuel | 2451: 154 (Nishida, m)<br>2490: 763 (Navypedia, m) | 2490 | Bulk | 2% |

### US escorts, Task Forces 11 and 16

**us-north-carolina-class** (North Carolina)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 36600: 453 (Wikipedia, l)<br>37484: 904 (Navypedia, m) | 37484 | Bulk | 2% |
| Speed | 28: 459 (Wikipedia, l), 905 (Navypedia, m) | 28 | Fair |  |
| Endurance | 17450: 455 (Wikipedia, l; at 15 kn), 906 (Navypedia, m; at 15 kn)<br>16320: 454 (Navy fuel tables, m; tanks 95% full, at 15 kn) | 16320 | Fair | 7% |
| Fuel | 7554: 457 (Navy fuel tables, m; tanks 95% full)<br>6260: 908 (Navypedia, m) | 7554 | Fair | 21% |

**us-atlanta-class** (Atlanta, San Juan)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 6718: 303 (Wikipedia, l)<br>6590: 909 (Navypedia, m)<br>6000: 914 (NavSource, m) | 6590 | Coin-toss | 12% |
| Speed | 32.5: 307 (Wikipedia, l), 913 (Navypedia, m), 915 (NavSource, m) | 32.5 | Fair |  |
| Endurance | 8500: 304 (Wikipedia, l; at 15 kn), 910 (Navypedia, m) | 8500 | Fair |  |
| Fuel | 1360: 912 (Navypedia, m) | 1360 | Fair |  |

**us-portland-class** (Portland)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 9950: 504 (uboat.net, m), 506 (Wikipedia, l), 922 (NavSource, m)<br>10096: 505 (Wikipedia, l)<br>10258: 916 (Navypedia, m) | 9950 | Fair | 3% |
| Speed | 32.5: 513 (uboat.net, m), 917 (Navypedia, m)<br>32.7: 514 (Wikipedia, l), 923 (NavSource, m)<br>30.8: 841 (Navy fuel tables, h; estimate at war-service weight) | 32.5 | Bulk | 1% |
| Endurance | 7210: 507 (Navy fuel tables, m; tanks 95% full, at 20 kn)<br>8640: 836 (Navy fuel tables, h; full load, at 15 kn)<br>9790: 838 (Navy fuel tables, h; tanks 95% full, at 15 kn)<br>10000: 918 (Navypedia, m; at 15 kn) | 7210 | Fair | 39% |
| Fuel | 3061: 509 (Navy fuel tables, m; tanks 95% full)<br>1500: 510 (Wikipedia, l)<br>2614: 840 (Navy fuel tables, h; tanks 95% full)<br>2125: 920 (Navypedia, m) | 3061 | Fair | 104% |

**uk-leander-modified-class** (Hobart)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 6830: 286 (uboat.net, m)<br>7003: 287 (Wikipedia, l)<br>7270: 288 (Wikipedia, l)<br>7105: 972 (Navypedia, m) | 6830 | Coin-toss | 6% |
| Endurance | 5730: 289 (Wikipedia, l; at 13 kn)<br>7000: 974 (Navypedia, m) | 7000 | Coin-toss | 22% |
| Fuel | 1837: 976 (Navypedia, m) | 1837 | Fair |  |

**us-farragut-class** (Farragut, Dewey, MacDonough, Worden, Dale)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1365: 351 (Wikipedia, l), 928 (NavSource, m)<br>1395: 350 (uboat.net, m)<br>1358: 932 (Navypedia, m) | 1365 | Bulk | 3% |
| Speed | 36.5: 929 (NavSource, m), 933 (Navypedia, m)<br>36: 358 (uboat.net, m)<br>37: 359 (Wikipedia, l)<br>31.6: 849 (Navy fuel tables, h; estimate at war-service weight) | 36.5 | Bulk | 3% |
| Endurance | 5980: 352 (Navy fuel tables, m; tanks 95% full, at 12 kn), 353 (Wikipedia, l; at 12 kn)<br>6500: 930 (NavSource, m; at 12 kn), 934 (Navypedia, m)<br>5240: 846 (Navy fuel tables, h; tanks 95% full, at 15 kn) | 5980 | Fair | 24% |
| Fuel | 608: 355 (Navy fuel tables, m; tanks 95% full)<br>607: 848 (Navy fuel tables, h; tanks 95% full)<br>600: 936 (Navypedia, m) | 608 | Bulk | 1% |

**us-porter-class** (Selfridge, Phelps, Balch)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1825: 483 (uboat.net, m)<br>1850: 484 (Wikipedia, l)<br>1834: 940 (Navypedia, m) | 1825 | Bulk | 1% |
| Speed | 37: 491 (uboat.net, m), 492 (Wikipedia, l), 937 (NavSource, m), 941 (Navypedia, m)<br>33.5: 853 (Navy fuel tables, h; estimate at war-service weight)<br>33.8: 857 (Navy fuel tables, h; estimate at war-service weight) | 37 | Bulk |  |
| Endurance | 6380: 485 (Navy fuel tables, m; tanks 95% full, at 12 kn), 486 (Wikipedia, l; at 12 kn)<br>6500: 938 (NavSource, m; at 12 kn), 942 (Navypedia, m; at 12 kn)<br>5670: 850 (Navy fuel tables, h; tanks 95% full, at 15 kn)<br>5620: 854 (Navy fuel tables, h; tanks 95% full, at 15 kn) | 6380 | Fair | 16% |
| Fuel | 651: 852 (Navy fuel tables, h; tanks 95% full), 856 (Navy fuel tables, h; tanks 95% full)<br>675: 488 (Navy fuel tables, m; tanks 95% full)<br>635: 944 (Navypedia, m) | 651 | Fair | 6% |

**us-gleaves-class** (Grayson, Monssen, Aaron Ward, Buchanan, Farenholt)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1620: 366 (uboat.net, m)<br>1630: 367 (Wikipedia, l)<br>1838: 963 (Navypedia, m) | 1620 | Coin-toss | 13% |
| Speed | 35: 964 (Navypedia, m), 968 (NavSource, m)<br>37: 374 (uboat.net, m)<br>37.4: 375 (Wikipedia, l)<br>31.9: 878 (Navy fuel tables, h; estimate at war-service weight) | 35 | Fair | 7% |
| Endurance | 6500: 369 (Wikipedia, l; at 12 kn), 969 (NavSource, m; at 12 kn)<br>5250: 368 (Navy fuel tables, m; tanks 95% full, at 12 kn)<br>4730: 875 (Navy fuel tables, h; tanks 95% full, at 15 kn)<br>3880: 965 (Navypedia, m) | 5250 | Fair | 68% |
| Fuel | 503: 371 (Navy fuel tables, m; tanks 95% full)<br>467: 877 (Navy fuel tables, h; tanks 95% full)<br>453: 967 (Navypedia, m) | 503 | Fair | 11% |

**us-benham-class** (Benham, Ellet, Lang, Stack, Sterett)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1657: 612 (Navypedia, l/i) | 1657 | Weak: one source |  |

**us-bagley-class** (Bagley, Patterson)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1624: 625 (History of War, l), 315 (Wikipedia, l) | 1624 | Weak: one source |  |
| Fuel | 544: 320 (Navy fuel tables, m; tanks 95% full)<br>506: 864 (Navy fuel tables, h; tanks 95% full)<br>504: 949 (Navypedia, m) | 544 | Fair | 8% |

**us-gridley-class** (Maury)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1590: 384 (Wikipedia, l), 950 (Navypedia, m)<br>1860: 383 (uboat.net, m) | 1590 | Coin-toss | 17% |
| Speed | 38.5: 392 (Wikipedia, l), 951 (Navypedia, m), 955 (NavSource, m)<br>38.6: 391 (uboat.net, m)<br>35.2: 868 (Navy fuel tables, h; estimate at war-service weight) | 38.5 | Bulk | 0% |
| Endurance | 5520: 385 (Navy fuel tables, m; tanks 95% full, at 12 kn), 386 (Wikipedia, l; at 12 kn)<br>6500: 952 (Navypedia, m), 956 (NavSource, m; at 12 kn)<br>4910: 865 (Navy fuel tables, h; tanks 95% full, at 15 kn) | 5520 | Fair | 32% |
| Fuel | 525: 867 (Navy fuel tables, h; tanks 95% full), 954 (Navypedia, m)<br>563: 388 (Navy fuel tables, m; tanks 95% full) | 525 | Fair | 7% |

### Japanese carrier screen, Vanguard Force and Ryujo group

**jp-kongo-class** (Hiei, Kirishima)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 31219: 85 (Nishida, m)<br>32350: 765 (Navypedia, m)<br>31980: 766 (Navypedia, m) | 32350 | Coin-toss | 4% |
| Fuel | 6230: 87 (Nishida, m)<br>6330: 771 (Navypedia, m) | 6330 | Bulk | 2% |

**jp-tone-class** (Chikuma, Tone)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 11038: 229 (Nishida, m)<br>11215: 773 (Navypedia, m) | 11215 | Bulk | 2% |

**jp-mogami-class** (Kumano, Suzuya)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Speed | 35: 783 (Navypedia, m), 784 (US technical mission, h)<br>34.9: 100 (Nishida, m) | 35 | Bulk | 0% |
| Endurance | 8150: 785 (Navypedia, m; at 14 kn)<br>7673: 787 (US technical mission, h; at 18 kn) | 7673 | Fair | 6% |
| Fuel | 1243: 98 (Nishida, l)<br>2302: 789 (Navypedia, m)<br>2310: 790 (US technical mission, h) | 2310 | Fair | 86% |

**jp-nagara-class** (Nagara, Yura)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 5482: 129 (Nishida, m)<br>5570: 792 (Navypedia, m) | 5570 | Bulk | 2% |
| Fuel | 1240: 130 (Nishida, m)<br>1260: 797 (Navypedia, m) | 1260 | Bulk | 2% |

**jp-kagero-class** (Akigumo, Amatsukaze, Hatsukaze, Hayashio, Isokaze, Kagero and 7 more)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 2001: 61 (Nishida, m)<br>2033: 799 (Navypedia, m) | 2033 | Bulk | 2% |
| Fuel | 500: 805 (Navypedia, l) | 500 | Weak: one source |  |

**jp-yugumo-class** (Kazagumo, Makigumo, Yugumo)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 2044: 251 (Nishida, m)<br>2077: 806 (Navypedia, m) | 2077 | Bulk | 2% |
| Fuel | 500: 811 (Navypedia, l) | 500 | Weak: one source |  |

**jp-akizuki-class** (Akizuki)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 2658: 4 (Nishida, m)<br>2701: 812 (Navypedia, m) | 2701 | Bulk | 2% |
| Speed | 33: 9 (Nishida, m), 814 (Navypedia, m)<br>33.7: 815 (US technical mission, h) | 33 | Bulk | 2% |
| Endurance | 8300: 5 (Nishida, m), 816 (Navypedia, m; at 18 kn)<br>9062: 818 (US technical mission, h; at 18 kn) | 8300 | Fair | 9% |
| Fuel | 1080: 7 (Nishida, m)<br>1097: 820 (Navypedia, m)<br>1071: 821 (US technical mission, h) | 1071 | Bulk | 2% |

### Advance Force, convoy and the absent Wasp group (lower priority)

**jp-myoko-class** (Haguro, Myoko)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Endurance | None |  | **Not found.** See `gaps.csv` |  |

**jp-nagato-class** (Mutsu)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Endurance | 8650: 143 (Wikipedia, l) | 8650 | Weak: one source |  |

**jp-chitose-class** (Chitose)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | None |  | **Not found.** See `gaps.csv` |  |
| Speed | 28.9: 37 (Nishida, l) | 28.9 | Weak: one source |  |
| Endurance | None |  | **Not found.** See `gaps.csv` |  |
| Fuel | None |  | **Not found.** See `gaps.csv` |  |

**jp-sendai-class** (Jintsu)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 5113: 166 (Nishida, m)<br>5195: 167 (Wikipedia, l) | 5113 | Bulk | 2% |
| Endurance | 5000: 168 (Wikipedia, l) | 5000 | Weak: one source |  |

**jp-shiratsuyu-class** (Harusame, Kawakaze, Murasame, Samidare, Suzukaze, Umikaze)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Endurance | None |  | **Not found.** See `gaps.csv` |  |

**jp-fubuki-class** (Shikinami, Uranami)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Endurance | None |  | **Not found.** See `gaps.csv` |  |

**jp-mutsuki-class** (Mutsuki, Uzuki, Yayoi)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | 1294: 106 (Nishida, m)<br>1315: 107 (Wikipedia, l) | 1294 | Bulk | 2% |
| Endurance | 3600: 108 (Wikipedia, l) | 3600 | Weak: one source |  |

**us-wasp-class** (Wasp)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Endurance | 12000: 543 (Wikipedia, l; at 15 kn) | 12000 | Weak: one source |  |
| Fuel | None |  | **Not found.** See `gaps.csv` |  |
| Aircraft capacity | 84: 534 (uboat.net, m)<br>100: 535 (Wikipedia, l) | 84 | Coin-toss | 19% |

**us-pensacola-class** (Salt Lake City)

| Fact | Rows by value | Recommend | Sure | Spread |
| --- | --- | --- | --- | --- |
| Displacement | None |  | **Not found.** See `gaps.csv` |  |
| Speed | None |  | **Not found.** See `gaps.csv` |  |
| Endurance | None |  | **Not found.** See `gaps.csv` |  |
| Fuel | None |  | **Not found.** See `gaps.csv` |  |

**Read these with care.**

- **Saratoga's displacement** is 36,000 (the treaty design figure), 37,681 or 40,000 depending on source and date. No source read gives her displacement in August 1942. The policy picks 40,000 on source count; it is a coin-toss.
- **Enterprise's fuel** is 4,280 or 4,360 by two sources and 6,500 by the ship's own website, which may be quoting a different condition. The policy picks on grade and the result is a coin-toss. The fuel tables have no section for her class.
- **Endurance at mixed speeds.** Saratoga's design figure is at 10 knots and Shokaku's at 18. A cruise speed of 10 knots for a carrier is not how she was run. The rule may want one cruise speed for all carriers and a consumption curve, which the fuel tables give for US ships only.
- **Japanese classes with no endurance at all:** Myoko, Fubuki, Shiratsuyu, Chitose. Four more have one weak row. These are in `gaps.csv`.
- **Pensacola class** (Salt Lake City, in the Wasp group) has no rows of any kind. The Wasp group was away fuelling on the day.

## 2. Anti-aircraft guns on 24 August (`fits.csv`, slots `heavy_aa` and `light_aa`)

The tool will be extended to read these. What the sources give:

| Ship | Rows | What each is | Recommend | Sure |
| --- | --- | --- | --- | --- |
| North Carolina | 489 to 492 (Navy narrative, h); 493 to 496 (Navypedia, m) | The narrative lists the guns "going" at the height of the attack: twenty 5-inch, four quadruple 1.1-inch (16 barrels), forty 20 mm, twenty-six .50. Navypedia gives five quadruple 1.1-inch and twenty-eight .50 | The narrative: 20, 16, 40, 26 | Fair. It says "almost all her guns", so it may undercount by a mount |
| Enterprise | 500 to 509 | Eight 5-inch (two sources agree). 1.1-inch: twenty barrels, two sources. 20 mm: 40 by the ship's website, 32 and 38 by Navypedia at two dates. .50: none by the website, but her own action report records 880 rounds of .50 fired | 8, 20, 38, and .50 present in a number not found | Coin-toss on 20 mm between 38 and 40. Bulk |
| Saratoga | 514 to 518 | Sixteen 5-inch (two sources). Light guns from one source, inferred from a dated list | 16; light guns 20 and 30 by exception | Weak for the light guns |
| Atlanta | 519, 521 | Sixteen 5-inch, sixteen 1.1-inch, one source | As given | Weak: one source |
| Portland | 524 to 527 | Eight 5-inch/25, sixteen 1.1-inch, twelve 20 mm, one source | As given | Weak: one source |
| Shokaku | 457, 459 and others | Sixteen 12.7 cm. 25 mm: at least 42 (Kojinsha, by inference), 48 (Navypedia) or 54 (Wikipedia) in August 1942 | 48 | Coin-toss. 29% apart |
| Ryujo | 467 to 470 | Twenty-two 25 mm (two sources, both low). 13 mm: 24 or none | 22; 13 mm unknown | Weak |
| Destroyers and other cruisers | class rows | Class fits as built or by refit year. **No source gives any destroyer's own fit on 24 August** | The class row that holds on the date | Fair for main and heavy guns, weak for light guns |

## 3. Radar on 24 August (`fits.csv`)

| Ship | What the sources say | Recommend | Sure |
| --- | --- | --- | --- |
| Enterprise | CXAM air search set: the ship's website and Navypedia agree. Her action report and the Navy narrative describe an air search radar in use (the 88-mile contact) without naming it. A fire-control radar is in her report. Navypedia adds SC and FD sets, disputed | CXAM | Fair |
| Saratoga | CXAM by two sources; SC and two FD by Navypedia. The narrative says her radar also held the raid | CXAM | Fair |
| North Carolina | CXAM by two sources; fire-control sets by Navypedia | CXAM | Fair |
| Atlanta | SC, SG and two FD, by Navypedia only | SC and SG, by exception | Weak: one source. If right, she had the only surface search set in the force |
| Portland | SC by Navypedia; fire-control sets by two weak sources | SC, by exception | Weak |
| US destroyers | Class-level refit dates only | Unknown | For the "unknown means not fitted" list, as at Savo |
| Shokaku | Type 21 air search, fitted June 1942 and used on 24 August: three rows, two sources | Type 21 | Fair |
| Zuikaku | None: fitted December 1942 by her movement record, and the Kojinsha volume says none at Santa Cruz in October. Navypedia says fitted in 1942 | None | Fair |
| Ryujo, Hiei | None, on one source that simply lists no radar | None, by exception | Weak |
| Kirishima | A Type 21 by one page of a website, none by another source | Unknown | Coin-toss |
| Tone, Chikuma, Kumano, Suzuya, Nagara | None: each ship's movement record dates her first set to November 1942 or later | None | Fair |

## 4. Aircraft (`aircraft_type_facts.csv` and `fits.csv`)

**What was found.** The best single document is the Navy's own data sheet for the F4F-4, which states a combat radius and how it was worked out. For the other US types no official sheet was found, and for the Japanese types the figures are wartime US intelligence or modern websites. What was actually flown on the day is recorded from the Navy narrative and the Enterprise report, and is the firmest evidence of radius in practice.

| Type | Radius or range | Rows | Recommend | Sure |
| --- | --- | --- | --- | --- |
| F4F-4 | **Combat radius 105 nautical miles clean, 245 with one 58-gallon drop tank, 325 with two** (Navy data sheet of July 1943). Cruise 140 knots | 244 to 247 | The sheet's figures. Whether fighters carried tanks on 24 August was not found | Fair. The sheet is ten months after the battle |
| SBD-3 | No official radius. **Flown on the day:** dawn search to 200 miles, afternoon search to 250 miles with a 500 lb bomb, strike with a 1,000 lb bomb at about 190 miles. Range 826 to 1,191 by two weak sources | 280 to 287; 226, 270, 271 | Search radius 250 with 500 lb; strike radius 200 with 1,000 lb, as flown | Fair for what was flown. It is a floor, not a limit |
| TBF-1 | No official radius. **Flown on the day:** search to 250 miles with bombs; strike with a torpedo at about 190 miles. 225 by one unsourced website | 288, 309 to 312 | 250 search, 200 strike, as flown | Fair for what was flown |
| A6M2 Zero | Radius 261, 434 or 521 nautical miles by one website quoting a book, for carrier, land-based and extreme cases. Range 1,000 to 1,050 by four sources including two captured-aircraft reports; 1,677 with drop tank by three | 361 to 363; 30, 371, 382, 385 | Carrier radius 261; range 1,010 clean | Coin-toss on radius: one secondary source |
| D3A1 Val | **No combat radius found.** Range 730 to 795 by three weak sources; 391 by a wartime US estimate. The first wave flew about 200 miles to its target | 98, 99, 393, 400 | Range 760; radius as flown, about 200 | Weak |
| B5N2 Kate | **No combat radius found.** Range 528 with torpedo (two weak sources), 1,075 maximum | 79, 412; 78, 408, 413 | Range 528 with torpedo | Weak |
| PBY-5 and 5A | No radius. Range 1,245 normal, 2,190 or 2,694 maximum, from one unsourced website and Wikipedia. | 314, 324; 215, 313, 323 | Range 2,190; search radius as flown | Weak |
| H6K4 Mavis | Range 1,938 (two rows, one source), 2,590, 3,284 | 186, 420, 187, 426 | 2,590 | Coin-toss |
| B-17E | Range 1,221 with bombs and 2,659 maximum, from Army Air Forces reports of 1941 to 1942 | 341, 342 | The Army figures | Fair |

**Guns and loads.** The pass added 60 fit rows. The F4F-4's six .50 guns with 240 rounds each come from the Navy sheet. The SBD-3 now has guns and bomb loads; the loads flown on the day (500 lb on search, 1,000 lb on strike) are from the Navy narrative. The TBF-1's guns agree between two sources; one torpedo or bombs. The D3A1's bomb load is 250 kg plus two 60 kg by most sources and 1,100 lb by one wartime US bulletin. **Recommend the loads flown on the day for US types and one 250 kg bomb for the Val, one torpedo for the Kate. Sure: fair.**

**Speeds.** Maximum speed is disputed for every type, usually within 3 per cent; the Zero has twelve rows from 273 to 299 knots. **Recommend, in bulk, the figure from the official or captured-aircraft document where there is one** (F4F-4 278, Zero 283 from the Navy trial digest, B-17E 276) and the most-quoted figure otherwise.

## 4a. Candidates for a measure of how much punishment an aircraft takes

You asked for candidates and not a choice. The facts layer had nothing; it now has three attributes, `pilot_armour`, `self_sealing_tanks` and `structural_note`, beside `empty_weight_kg`.

| Candidate | Sourced for | From | Weak or missing for |
| --- | --- | --- | --- |
| Pilot armour, yes or no | F4F-4 yes (about 160 lb, in the maker's specification and the Navy sheet). SBD-3 yes. A6M2 no (four wartime US documents on a captured aircraft) | Official documents | TBF-1 and B-17E yes by inference only. D3A1, B5N2, H6K4 no, from Wikipedia or one website |
| Self-sealing fuel tanks, yes or no | F4F-4 yes. SBD-3 yes. A6M2 no. D3A1 no (five unprotected tanks, one website quoting a book) | Official documents for the first three | TBF-1, B-17E, PBY inferred. B5N2 and H6K4 no, weak |
| Empty weight | F4F-4 (2,621 to 2,674 kg, official). A6M2 (1,715 kg, from the data plate of the captured aircraft) | Official documents | SBD-3, D3A1, B5N2, PBY: websites only. All disputed within a few per cent |
| What an official evaluation said | F4F-4 "rugged"; A6M2 light construction, skin 0.02 inch | Navy and Army reports | Nothing for the D3A1 or PBY |
| Loss rate in a named action | Japanese first wave: 24 of 37 lost. Saratoga's strike on Ryujo: none or one of 36 | Section 7 | These are results of the battle, so using them to set toughness would be circular for this scenario |

The first two are yes-or-no facts with official sources for the three types that matter most and weak ones for the rest. Empty weight is a number for every type but separates a fighter from a bomber more than a tough aircraft from a fragile one.

## 5. Air strengths on 24 August (`air_unit_facts.csv`)

**What was found.** One website, a weapons reference site's order of battle, gives aircraft on hand and operational for every squadron on both sides. It cites two books that were not read. It agrees with Wikipedia on most squadrons, and its fighter figures add up exactly to the only primary total, the Navy narrative's 54 fighters on the two US carriers. Because two sources agree, most of these rows are `preferred` by rule already.

| Unit | On hand | Operational | Check against a primary source | State |
| --- | --- | --- | --- | --- |
| Saratoga: VF-5, VB-3, VS-3, VT-8 | 27, 17, 15, 13 | 27, 15, 15, 13 | The Navy narrative of the landings gives her 27 fighters, 33 dive bombers and 15 torpedo planes on 8 August; the Eastern Solomons narrative has at least 32 dive bombers and 13 torpedo planes flyable | Preferred by rule. Consistent |
| Enterprise: VF-6, VB-6, VS-5, VT-3 | 28 (29 by Wikipedia), 17, 18, 15 | 27, 17, 16, 14 | Her war diary has at least 33 dive bombers and 13 torpedo planes flyable. 27 operational fighters with Saratoga's 27 makes the narrative's 54 | VF-6 on hand is disputed, 28 or 29. **Recommend 28 on hand, 27 operational. Sure: fair** |
| Shokaku | 27 fighters, 27 dive bombers, 18 attack planes | 26, 27, 18 | Her movement record has 27 dive bombers launched over two strikes | Preferred by rule |
| Zuikaku | 27, 27, 18 | 25, 27, 18 | 27 dive bombers launched | Preferred by rule |
| Ryujo | 24 fighters; attack planes 9 (her record, 16 August) or 6 (the website, 24 August) | 23; 6 | Six attack planes and 14 fighters launched against Henderson Field | Attack planes disputed. **Recommend 9 on hand, 6 operational. Sure: coin-toss** |
| Henderson Field | VMF-223: 19 arrived 20 August, 13 on the 24th. VMSB-232: 12, then 11. 67th Fighter Squadron: 5. Enterprise's Flight 300: 11 dive bombers from the evening | | The Navy narrative: about 20 fighters and 12 dive bombers | **Recommend the 24 August figures: 13, 11, 5, and 11 after dusk. Sure: fair** |
| Rabaul | 114 aircraft by type | | None | One source. Weak |

Wasp's squadrons have one source each and she was absent.

## 6. Where the Japanese forces were (`engagement_positions.csv`)

**What can be sourced from the Japanese side, and it is thin:**

- **A post-war Japanese account** (Monograph 98) gives positions by day, in words: both fleets 200 to 400 miles north of the Solomons on the 23rd; the carriers about 300 miles north-north-east of Tulagi on the 24th with the Advance Force 60 miles east of them.
- **The post-war US survey's track chart** of the Japanese forces. Sixteen points were read off it by eye, so each is good to about 12 miles and is graded low. It has the carriers near 5°S 161.5°E at noon on the 24th, the Ryujo group near 4.7°S 161.5°E at 0900, and the Advance Force turning north near 8.3°S 163.2°E at 2200.
- **Ship movement records** give times for a few events and almost no positions.

**What has to be worked back from US sighting reports, already in the data:** the Ryujo group at 0935 and 1606, the carriers at 1500 (198 miles from the US force on bearing 340), the cruisers at 1510 and 1805. Each is a report, marked as one.

**How far the two disagree where both exist:** about 15 miles for Ryujo in the morning, 45 to 50 miles for Ryujo in the afternoon and the carriers at 1500. Some of that is the half-hour difference between the two sides' clocks.

**Not found anywhere:** where Ryujo was detached and when (four times are given, from midnight to 0400), where she sank, where the carriers were when they launched, and any position at all for Hiei, Kirishima, Kumano or Suzuya.

**Recommend for the scenario's start:** place the carriers and the Ryujo group from the track chart, flagged as read by eye, and check the result against the 1500 and 0935 US reports. **Sure: fair to within 50 miles, which is about one hex.**

## 7. Strike results, for calibration later (`engagement_outcomes.csv`)

Not needed to run the scenario. The pass added 108 rows, each marked as the attacker's claim or the target's record. The pairs that matter:

| Strike | Aircraft | Hits claimed | Hits by the target's record | Aircraft lost |
| --- | --- | --- | --- | --- |
| Japanese first wave on Enterprise | 27 dive bombers, 10 fighters | | 3 bombs | 18 dive bombers and 6 fighters by the Japanese record; US fighters and guns claimed about 70 |
| Japanese second wave | 27 dive bombers, 9 fighters | Did not find the target | | 5, landing after dark |
| Saratoga on Ryujo | 30 dive bombers, 8 torpedo planes | 4 to 10 bombs, 1 torpedo | 4 bombs and 1 torpedo by one Japanese record | None, or one torpedo plane |
| Enterprise search planes | 2 on the carriers, 2 on a cruiser | Near misses | Near misses; 6 killed in Shokaku | |
| Henderson Field and B-17s, 25 August | 6, 8 or 12 dive bombers; 8 or 9 B-17s | | Jintsu 1 bomb, Kinryu Maru sunk, Mutsuki sunk | |

No source gives bombs dropped by the Japanese strikes. Losses are split by cause only where a source does so; the Enterprise report estimates 15 shot down by ships' guns and 28 by her fighters, against 24 lost in all.

## 8. Not found

All of these are in `gaps.csv`.

- **Official performance sheets** for the SBD-3, TBF-1 and PBY, and any combat radius for the D3A1, B5N2, H6K4, PBY or B-17E.
- **Japanese bomb counts,** and which fighters carried drop tanks.
- **A destroyer's own anti-aircraft or radar fit** on 24 August, for either side.
- **Squadron strengths from a primary source.** The weekly "Location of US Naval Aircraft" reports would give them; the Navy's history site refuses automated reading.
- **Endurance** for the Myoko, Fubuki, Shiratsuyu and Chitose classes, and everything for the Pensacola class.
- **The Advance Force and convoy classes** were not researched in this pass beyond what was already held.
- **Saratoga's own action report.**

## What approval would do

If the three questions in section 0 and the recommendations are approved, the rows are marked as at Savo Island: the chosen row `preferred` where the format allows it, the others `alternative`, each with a note naming the item. Rows chosen that rest on a lone weak source are listed for a written exception. The derivation tool does not yet read aircraft, air strengths or anti-aircraft fits, so its problem count for this battle cannot be given.
