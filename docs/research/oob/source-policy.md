# Order-of-battle source policy

Status: proposal, for review by Amit. It governs what may be entered in `data/oob/` and how it is cited.

This is a working policy written by a non-lawyer. It is cautious on purpose. If the repo is made public, the "needs a decision" items at the end should be settled first.

## Principles

1. **Facts are free; expression is not.** A ship's commissioning date or a gun's calibre is a fact and cannot be owned. A source's prose, and the particular selection and arrangement of a table it compiled, can be. US law gives a compilation only thin protection, limited to its original selection and arrangement (Feist v. Rural, 499 U.S. 340, 1991). The UK, and by the same model the EU, also protect a database against extraction of a substantial part where there was substantial investment in building it, for 15 years (UK Copyright and Rights in Databases Regulations 1997, regs. 13 and 17).
2. **So: take facts one at a time, into our own structure.** No table is copied wholesale. No single copyrighted compilation is mined systematically for most of what it holds. Where a class of facts comes mostly from one compilation, a second source is sought.
3. **Our own words.** Notes, summaries and the `note` column are written by us. At most one short quotation per note, marked and attributed.
4. **Cite what was read.** A row cites the document or page actually opened, with the date. A book quoted by a website is cited as the website "citing" the book, never as the book's page.
5. **Nothing from any wargame.** No figure, rating, table or list from War in the Pacific: Admiral's Edition, its manuals, its mods, or any other game. A game's data is someone's ratings layer, not a fact.
6. **Nothing invented.** A fact with no source is not entered. It goes in `gaps.csv`.
7. **Disagreement is recorded.** See "Conflicting sources" in `database-design.md`.

## Source classes

Reliability grades: **A** is a primary record of the side that owned the thing, or a careful modern study built on both sides' records. **B** is a competent secondary work or compilation, usually right, sometimes dated or unsourced. **C** is useful as a lead only: wartime estimates of the enemy, memory-based accounts, tertiary summaries.

The "rights" column says what we believe. The "checked" column says whether the site's own terms were read on 2026-11-10. Several sites refused automated reading that day, and those are marked honestly as not checked.

### Use freely: US government works

Works prepared by US government employees as part of their duties have no copyright in the United States (17 U.S.C. § 105). They may be quoted and their facts used without limit. Outside the US the position is less clear, which matters little for facts.

| Source | Reliability | Notes | Rights checked? |
| --- | --- | --- | --- |
| Dictionary of American Naval Fighting Ships (DANFS), Naval History and Heritage Command | B for dates and movements. Older entries are brief and sometimes wrong on detail | First choice for US ship dates | No. history.navy.mil would not load. Treated as a US government work on authorship |
| Other NHHC publications and pages | A to B | | Partly. A search summary of NHHC's copyright page says official documents are public domain. Photographs and personal papers may not be |
| ONI Combat Narratives, 1943 (Savo Island, Eastern Solomons, and others) | A for what the US side did and knew. C for anything about the Japanese: written in wartime | First choice for US task organisation | Government work on authorship |
| Naval War College analyses (Bates, *The Battle of Savo Island*, 1950) | A. Used Japanese records and interrogations | The most detailed account of Savo | No rights statement on the hosted copy. Government work on authorship |
| Bureau of Ships War Damage Reports | A | Damage, fires, what was aboard | Government work on authorship |
| Wartime technical manuals: FTP 217 *U.S. Radar: Operational Characteristics* (1943), ordnance pamphlets, aircraft characteristics sheets | A for nominal performance. Stated performance is not performance achieved in action | | Government work on authorship |
| CINCPAC "Gray Book" and action reports | A for what was known and ordered at the time | | Hosted copy carries a society's name but no rights claim. Government work on authorship |
| United States Strategic Bombing Survey: *Interrogations of Japanese Officials*, *The Campaigns of the Pacific War* | B. Interrogations are memory under questioning. Good for Japanese organisation and intent | | No rights statement on the hosted copy. Government work on authorship |
| US Naval Technical Mission to Japan reports, 1945 to 1946 | A for Japanese equipment as examined after the war | First choice for Japanese radar, sonar, fire control and torpedoes | No rights statement on the hosted copies. Government work on authorship |
| Japanese Monographs (US Army Far East Command, written by former Japanese officers) | B to C. Often written from memory after records were lost | Prepared for and published by the US Army | No rights statement. The authors were not US employees, so the status is less certain than for the rows above. Use the facts, do not reproduce the text |
| US Army, Marine Corps and Navy official histories published by the government (Miller, *Guadalcanal: The First Offensive*; Zimmerman, *The Guadalcanal Campaign*; *Pearl Harbor to Guadalcanal*) | B. Sound, sometimes superseded on Japanese detail | | No. history.army.mil refused the request. Generally treated as public domain. Credited outside photographs may differ |
| Wartime recognition manuals (ONI 41-42, ONI 222-J) and TAIC aircraft manuals | C. Estimates of enemy equipment, often wrong | Enter with `confidence` `low`. Their errors are themselves useful for the belief model | Government work on authorship |

**HyperWar** (ibiblio.org/hyperwar) is a host, not a source. It says its documents are almost all US government products, and that British and Commonwealth documents are Crown copyright. Cite the original document, give the HyperWar URL as where it was read, and remember a transcription can contain errors.

### Use the facts, cite, never copy: copyrighted works

| Source | Reliability | Notes | Rights checked? |
| --- | --- | --- | --- |
| Published reference books: Friedman's design histories and *Naval Radar*; Campbell, *Naval Weapons of World War Two*; Lacroix and Wells, *Japanese Cruisers of the Pacific War*; Jentschura, Jung and Mickel; Francillon, *Japanese Aircraft of the Pacific War*; Conway's | A to B | Cite author, title, edition, page. Only when someone has the book in hand | Copyrighted |
| Modern studies: Lundstrom, *The First Team and the Guadalcanal Campaign*; Frank, *Guadalcanal*; Evans and Peattie, *Kaigun*; Hata, Izawa and Shores | A | The best sources for air unit strengths and losses on both sides. Cite by page, in hand only | Copyrighted |
| Morison, *History of United States Naval Operations in World War II*, vols. IV and V | B. Readable, dated, weak on the Japanese side | Written by a serving officer but published commercially and treated as copyrighted | Copyrighted |
| Craven and Cate, *The Army Air Forces in World War II* | B | The reprint's front matter shows a University of Chicago copyright, renewed 1984, and also says the work is a product of the US government apart from editing. Treat as copyrighted | Yes, vol. VI front matter read |
| combinedfleet.com (Nihon Kaigun): tabular records of movement, class pages | B. The standard English-language record of Japanese ship movements. Sourcing is uneven page to page | No reuse statement found on the pages read. Treat as all rights reserved. Take individual dates, never a whole record | Yes, three pages read. No statement found |
| navweaps.com | B. Mostly drawn from Campbell and official publications, and says so | Site states it is copyrighted. Personal and educational use is allowed with credit; commercial use needs written permission | Yes |
| navsource.org | B for US ship data and photographs | Pages copyrighted to the site. Official US Navy images are public domain, donated ones are not | Yes, possibly an older notice |
| uboat.net (Allied warships section) | B | Copyright notice only. No reuse policy found | Yes |
| Senshi Sōsho, the Japanese official history (National Institute for Defense Studies) | A for the Japanese side | In Japanese. Online, with a copyright notice and no reuse licence. Publication use of archive material needs an application. Cite volume and page for facts | Yes. Which rule covers the scans is not stated |
| JACAR (Japan Center for Asian Historical Records): unit action reports, war diaries | A. Primary | In Japanese. Images may be downloaded for personal use. Reuse of National Archives and Foreign Ministry items is generally allowed with credit and reference code. Items held by the defence institute need contact before publication | Yes |
| Gill, *Royal Australian Navy 1942–1945* (Australian official history, 1968), at the Australian War Memorial | A for Australian ships | The Memorial's page carries an "all rights reserved" footer, and its general site text is CC BY-NC. Whether the 1968 volume itself is still in Crown copyright was not established | Yes. Status of the volume unclear |
| Royal Australian Navy ship histories (navy.gov.au) | B | Commonwealth of Australia copyright per search results | No. The site would not load |

### Leads only

| Source | Use |
| --- | --- |
| Wikipedia | Text is CC BY-SA 4.0 (and GFDL). A fact from it carries no licence, but the article is tertiary and changes. Use it to find the source it cites, then read and cite that. A row citing Wikipedia directly is `low` confidence and says what Wikipedia cites. Never copy its text or tables into the repo: share-alike terms would attach to the copy |
| Enthusiast sites and museum summaries | May be cited only when nothing better was found, always at `low` confidence, with reliability `C` in `sources.csv`. Each such row is a lead to be replaced |
| Forums, model-kit references, YouTube | Never cited in a fact row |
| Any wargame or its documentation | Not used at all |

## Reliability rules of thumb

- **The first data set leans on leads.** About 29% of its rows cite Wikipedia and a few cite enthusiast sites, because the Navy's history site would not load and no book was in hand. That is recorded, not hidden: those rows are `low`, and `data/oob/README.md` gives the counts.
- **Each side is the authority on itself.** US records for US ships; Japanese records, or studies built on them, for Japanese ships. A US wartime statement about Japanese equipment or losses is a claim, not a fact, and is `low`.
- **Claims are not losses.** Aircraft shot down and ships sunk "as reported" by the claiming side are belief, not truth. Only the loser's records establish a loss.
- **Nominal is not actual.** A manual's radar range or a gun's rate of fire is the stated figure. What was achieved in action belongs in the research notes, with its own sources.
- **Dates on a ship's equipment need a dated source.** A class having a radar does not show a ship had it on a given night. Without a dated source the row is `inferred` or it is a gap.
- **Two copies are one source.** Sites that repeat each other, or both repeat one book, do not corroborate.

## How to cite

In `sources.csv`, one row per work and edition: author, title, publisher, year, URL if online, rights, reliability, and a note on known weaknesses.

In each fact row:

| Field | Printed work | Online document | Web page |
| --- | --- | --- | --- |
| `source` | Key of the book | Key of the original document, not the host | Key of the site |
| `locator` | Page, and table or figure number | Full URL, plus section or page within it | Full URL |
| `accessed` | Empty | Date read | Date read |

A web page can change or vanish. For anything a design choice will rest on, prefer a dated document over a live page.

## If the repo is made public

| Material | Position |
| --- | --- |
| The facts in `data/oob/` | Ours to publish. Each is an individual fact with a citation, in our own structure |
| The database as a whole | Our own compilation. We can license it as we choose |
| Research notes | Our own prose. Fine, provided the quotation rule was kept |
| Text copied from US government works | Allowed in the US. We still avoid it, for consistency and because the position abroad is less clear |
| Anything copied from a copyrighted book or site | Not allowed, and the rules above mean there should be none |
| Text or tables from Wikipedia | Would bring CC BY-SA terms with it. The rules above mean there should be none |
| `references/` (the WitP:AE manuals) | Never committed, public or not |

A public repo should carry a short notice: sources are credited in `sources.csv`, facts are drawn individually from them, and anyone who believes a compiled table has been reproduced should say so and it will be re-sourced or removed.

## Needs a decision before going public

- **Heavy reliance on one site.** Japanese ship dates lean on combinedfleet.com and weapon figures on navweaps.com, because the books they draw on are not in hand. Each use is a fact with a citation, but the volume from one compilation is the thing database rights are about. Buying or borrowing the underlying books, and re-citing from them, removes the question.
- **Gill and other Australian official histories.** Establish whether the 1968 volume is out of Crown copyright before quoting it at length. Facts from it are fine now.
- **Japanese Monographs.** Establish their status before reproducing any passage.
- **Licence for our own data.** Not chosen. Open in `docs/pillars.md` ("Open source from the start, or decide after the kill gate?").

## What was not verified on 2026-11-10

Recorded so nobody reads the tables above as firmer than they are.

- history.navy.mil, history.army.mil and navy.gov.au could not be read by automated fetch. Their rights statements above come from search summaries or from authorship, not from the sites' own pages.
- No page was found stating outright that the Technical Mission reports, the Strategic Bombing Survey volumes, the Bates analysis or the Gray Book are public domain. They are treated as US government works because of who wrote them.
- The EU database directive could not be loaded. The UK regulations, which implement the same model, were read.
- The current, post-Brexit scope of the UK right was not checked.
