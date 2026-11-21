# Fan-made rules, iteration 1 — Investigators

## Setup

- Use the existing board and investigator starting positions. Each investigator starts with **$3** and an empty inventory for the item cards in this iteration.
- Shuffle the **Crossroads**, **Port**, and **Isolated Town** lieu decks separately. Each deck contains one copy of each of its **20** cards and has its own discard pile. All investigators share these decks.
- Set aside one copy of each of the **10 Unique** item cards in a reserve shared by all Magic Shops and lieu events. Use the **30 Common** item cards as a catalog and take a copy each time one is awarded or purchased.
- Sea zones are empty travel spaces. They have no lieu cards or shops.

## Movement

On their movement step, an investigator may stay in place or move along **one** board link. Use the existing road and sea connections. A road connects two cities. A sea link connects a port to a sea zone or two sea zones to each other. State-border roads work like any other road. Destroyed cities are closed to travel and local actions.

A movement item lets the investigator take a longer route as described on the card. Choose the whole route before moving and spend the item only if the route uses the extra movement. Follow the links in order, visiting each space at most once, including the starting space. The last space is where the investigator stops and, if it is an open city, where they can take a local action. If an investigator loses their turn, skip movement and the local action.

## City actions and lieu cards

At the point when the investigator would normally get a city event, choose **one** local action in their current city. An investigator who stayed in an open city can make the same choice.

1. **Draw an event.** Reveal the top card from the city's lieu deck, apply its effect to the investigator, then put it in that deck's discard pile. If the draw pile is empty when a card is needed, shuffle that deck's discard pile into a new draw pile.
2. **Buy an item.** Choose one card from the city's shop, pay its price, and add it to the investigator's inventory. If they buy nothing, they may still draw an event. A purchase uses the local action for that turn.

Use the existing city classification in this order. A city with **four or more road links** is a **Crossroads**, even if it is a port on the board. Of the remaining cities, a city marked as a port uses the **Port** deck. All other cities use the **Isolated Town** deck. This gives 20 crossroads, 14 ports, and 13 isolated towns. The deck depends on the city where the investigator stops. The shop type has no effect on the deck.

### Crossroads lieu deck

| ID | Card | What happens | Effect |
| --- | --- | --- | --- |
| `crossroads_01` | The Late Edition | A newspaper pays you to take a bundle to the newsstands around the square. | Gain $3. |
| `crossroads_02` | The Long Count | A dispatcher hires you to tally wagons through the afternoon rush. | Gain $2. |
| `crossroads_03` | A Tip for the Piano | You return a pianist's lost sheet music. She tips you from the collection jar. | Gain $1. |
| `crossroads_04` | The Garage Sweep | A mechanic lets you keep a kit salvaged from a wreck. | Gain a Road Kit. |
| `crossroads_05` | The Clerk's Spare | A survey clerk gives you a notebook with unused expense slips. | Gain a Surveyor's Notebook. |
| `crossroads_06` | The Surveyor's Wake | A widow gives you the compass that led her husband home through a vanished street. | Gain a Folding Compass. |
| `crossroads_07` | Cab to Nowhere | A cab stops a block from where it picked you up. The driver insists on a fare. | Lose $1. |
| `crossroads_08` | The Missing Tram | A tram scheduled for noon never arrives. | No effect. |
| `crossroads_09` | Three Keys at Closing | A stationmaster pays you to return three overdue locker keys. | Gain $4. |
| `crossroads_10` | The Last Delivery | A grocer pays you to carry a sack of flour after his delivery boy goes home. | Gain $3. |
| `crossroads_11` | Names in the Display | Every sample invitation in a printer's window announces your funeral. | Lose 1 SAN. |
| `crossroads_12` | An Idle Fleet | A dispatcher signs over a staff car that would otherwise sit in the depot. | Gain a Motor Pool Chit. |
| `crossroads_13` | The Quayside Coach | A stranded passenger gives you the through ticket for a trip they can no longer make. | Gain a Coach Transfer. |
| `crossroads_14` | Petty Cash | A field-office clerk hands you a blank reimbursement envelope. | Gain an Expense Envelope. |
| `crossroads_15` | Grand Opening | A hardware dealer hands you a voucher for the opening sale. | Gain a Hardware Coupon. |
| `crossroads_16` | Stop the Press | A compositor slips you a corrected sheet before the presses start again. | Gain a Second Edition. |
| `crossroads_17` | Scrap by the Siding | A yard worker pulls a bent spike from a stack of old sleepers and offers it to you. | Gain a Rail Yard Spike. |
| `crossroads_18` | Claim Check Thirty-One | You help a repairer recover his toolbox. He gives you a carbine a customer abandoned months ago. | Gain a Stub-Bore Carbine. |
| `crossroads_19` | The Taxi That Came Back | A mechanic salvages an odometer from a cab that returned with four hundred miles missing. | Gain an Unmarked Odometer. |
| `crossroads_20` | The Coat with No Owner | The cloakroom attendant hands you a purse that crinkles like wings. | Gain a Moth-Silk Purse. |

### Port lieu deck

| ID | Card | What happens | Effect |
| --- | --- | --- | --- |
| `port_01` | The Night Inventory | A dock foreman pays you to match a late shipment against his manifest. | Gain $3. |
| `port_02` | A Net Full of Bottles | You help a fisher haul up a fouled net and share the bottle deposits. | Gain $2. |
| `port_03` | The Old Horn | You return a lost foghorn to its owner for a small reward. | Gain $1. |
| `port_04` | The Spare Crossing | A deckhand pays back a favor with a prepaid crossing. | Gain a Ferry Voucher. |
| `port_05` | The Fishmonger's Marker | You find a runaway handcart and the market steward gives you a token. | Gain a Merchant's Token. |
| `port_06` | Midnight Soundings | A retired pilot entrusts you with a dial that tracks a second, hidden tide. | Gain a Tide Dial. |
| `port_07` | The Wet Manifest | You pay a clerk to replace papers ruined by a drenching wave. | Lose $1. |
| `port_08` | The Quay in Fog | The fog lifts, leaving the quay as quiet as before. | No effect. |
| `port_09` | A Crate Gone Adrift | A boatbuilder pays you to recover a crate of fittings before the tide turns. | Gain $4. |
| `port_10` | Whitewashing the Buoys | The harbor office pays by the buoy. You finish three before the rain. | Gain $3. |
| `port_11` | Voices from the Dry Dock | The pumps stop and a work crew answers roll call from beneath the empty dock. | Lose 2 SAN. |
| `port_12` | From the Landing to the Depot | The ticket clerk honors a missed connection with a transfer onto the road service. | Gain a Quay Transfer. |
| `port_13` | The Launch on Standby | A survey crew leaves behind a paid hire slip for its launch. | Gain a Launch Hire Slip. |
| `port_14` | The Broker's Favor | You retrieve a customs broker's papers. She writes you a letter of credit for an antiquarian. | Gain a Broker's Note. |
| `port_15` | A Refund in Brass | A ferry clerk offers a brass check redeemable on a future landing. | Gain a Brass Fare Check. |
| `port_16` | Between the Fish Stalls | A stallholder sees the crowd crush against you and offers a purse with a secure clasp. | Gain a Buttoned Purse. |
| `port_17` | Clearing the Pilings | A dockhand lends a long pike to pull flotsam away and lets you keep it. | Gain a Pier Pike. |
| `port_18` | Retired from the Lightship | A crewman gives you an obsolete signal pistol while clearing his locker. | Gain a Signal Pistol. |
| `port_19` | Light below the Surface | A lamplighter hauls up a lantern that still burns after a night underwater. | Gain an Undertow Lantern. |
| `port_20` | The Unclaimed Demonstration | An electrical salesman vanishes, leaving his strange projector packed for a demonstration. | Gain a Blue Arc Projector. |

### Isolated Town lieu deck

| ID | Card | What happens | Effect |
| --- | --- | --- | --- |
| `town_01` | The Failing Dynamo | You help a millwright get the lights back on before dusk. | Gain $3. |
| `town_02` | Apples for the Schoolhouse | The schoolkeeper pays you for bringing in crates before a storm. | Gain $2. |
| `town_03` | The Wrong Doorstep | A farmer rewards you for delivering a letter left at an empty house. | Gain $1. |
| `town_04` | The Spares in the Barn | The owner of a disused barn offers you a bundle of spare tools. | Gain a Road Kit. |
| `town_05` | The Cooperative Ledger | A shopkeeper returns your deposit in trade credit. | Gain a Merchant's Token. |
| `town_06` | The Uncovered Cabinet | A glassmaker leaves you a mirror whose reflection sometimes shows a different afternoon. | Gain a Smoked Mirror. |
| `town_07` | The Broken Latch | You pay for a latch that snaps as you open a borrowed gate. | Lose $1. |
| `town_08` | The Empty Grange | You follow a light to a grange hall and find it empty. | No effect. |
| `town_09` | The Orchard Account | A grower pays you to tally the harvest before the buyer arrives. | Gain $4. |
| `town_10` | The Missing Mailbag | You find the postman's canvas sack in a ditch and receive the reward. | Gain $3. |
| `town_11` | The Second Knocking | You knock at an abandoned farmhouse. The sound echoes back from inside your coat. | Lose 2 SAN. |
| `town_12` | The County Clerk's Shortcut | A clerk stamps a permit for a shortcut used by county vehicles. | Gain a County Motor Permit. |
| `town_13` | The Substitute List | The millkeeper signs a slip authorizing you to cover a paid shift on short notice. | Gain a Relief Shift Slip. |
| `town_14` | Blank Line on the Claim | A pawnbroker gives you a claim you can redeem against the next thing you find. | Gain a Pawnshop Claim. |
| `town_15` | The District Nurse | A nurse slips you a voucher for a consultation on her circuit. | Gain a Clinic Voucher. |
| `town_16` | The Hive Keeper | A beekeeper gives you wax plugs before she opens a hive humming out of season. | Gain Wax Earplugs. |
| `town_17` | Shutting the Toolroom | A mill foreman offers you a spare hammer before locking up for the winter. | Gain a Toolroom Hammer. |
| `town_18` | Hooked in the Branches | A picker gives you an old crook for helping bring down a ladder. | Gain an Orchard Crook. |
| `town_19` | Tuning the Parish Organ | An organ tuner gives you a fork whose note steadies your hands. | Gain an Ivory Tuning Fork. |
| `town_20` | The Bell without a Tongue | A sexton gives you the blade hammered from the clapper of a bell that rang on its own. | Gain a Bell-Clapper Blade. |

Money cannot fall below $0. SAN losses use the existing sanity rules. Each table's **Effect** column is the full rule for that event. Take named Common items directly from the catalog. Take named Unique items from the shared reserve if available, or gain **$2** if the item is already held. Declining an available item gives no cash.

For an item that replaces a revealed event, set the first card aside and draw the next card from the same deck, reshuffling that deck's discard pile if needed. Resolve the second card and discard the first card followed by the second. An item that asks you to look at two cards uses the same draw procedure, then follows its own instructions for the unchosen card.

## Shops and item cards

Each city has exactly one shop. **Dunwich, Arkham, Boston, Providence, Provincetown, New Bedford, Newport, and Edgartown** have a **Magic Shop**. Every other city has a **Regular Shop**. Regular Shops carry all priced Common items in unlimited stock. Magic Shops sell the Unique items currently in the shared reserve. A dash in the price column means the card is available through events.

An investigator can carry up to **three** item cards, including at most **one weapon**. Common gear may be carried in duplicate. When taking a new card, the investigator may discard one carried card to meet those limits. They can also decline a reward or cancel a purchase before paying. Remove spent or discarded Common cards from the inventory. Return a Unique item to the reserve when it is used up, discarded, or its owner is removed from play.

An investigator may spend **one gear card per turn**. Gear is single-use and spent only when its effect is applied. A spent card frees an inventory slot before any item is received. Effects on a revealed lieu card are chosen before applying that card. SAN restored by gear cannot go above the investigator's starting SAN. Apply any discount, spend the discount card, and pay as part of the confirmed purchase. Prices have a minimum of $0.

Weapons are reusable and use the existing weapon options in an encounter or fight. Add the listed bonus to the weapon roll against the named opponent. Use a maximum modified result of **6** against Ghouls and the Professor or **7** against Fanatics, including the existing medallion bonus. A weapon stays in the inventory after use and does not count toward the gear limit per turn.

### Common item cards (30)

| ID | Item | Type | Regular Shop | Effect |
| --- | --- | --- | ---: | --- |
| `road_kit` | Road Kit | Gear | $2 | During movement, take two consecutive road links. |
| `ferry_voucher` | Ferry Voucher | Gear | $2 | During movement, take two consecutive sea links. |
| `surveyors_notebook` | Surveyor's Notebook | Gear | $2 | When you resolve a lieu card with a printed cash reward, gain $3 more. |
| `merchants_token` | Merchant's Token | Gear | — | When buying an item, reduce its price by $2. |
| `motor_pool_chit` | Motor Pool Chit | Gear | $4 | During movement, take two or three consecutive road links. |
| `coach_transfer` | Coach Transfer | Gear | $2 | During movement, take one road link followed by one sea link. |
| `quay_transfer` | Quay Transfer | Gear | $2 | During movement, take one sea link followed by one road link. |
| `county_motor_permit` | County Motor Permit | Gear | $1 | During movement, take two consecutive road links, at least one of which crosses a state border marked on the board. |
| `launch_hire_slip` | Launch Hire Slip | Gear | $3 | If you start movement in a port, take three consecutive sea links. |
| `expense_envelope` | Expense Envelope | Gear | $1 | When you resolve a lieu card with a printed reward of $1 or $2, gain $4 instead. |
| `relief_shift_slip` | Relief Shift Slip | Gear | $1 | If the revealed lieu card says **No effect**, gain $3 instead. |
| `buttoned_purse` | Buttoned Purse | Gear | $1 | Ignore the cash loss on the revealed lieu card. |
| `hardware_coupon` | Hardware Coupon | Gear | — | When buying a Common item, reduce its price by $2. |
| `brokers_note` | Broker's Note | Gear | $2 | When buying a Unique item, reduce its price by $3. |
| `pawnshop_claim` | Pawnshop Claim | Gear | $2 | If the revealed lieu card grants a Common item, gain $3 instead of the item. |
| `carbon_copy` | Carbon Copy | Gear | $1 | If the revealed lieu card lists a cash loss, replace it with the next card. |
| `second_edition` | Second Edition | Gear | $1 | If the revealed lieu card says **No effect**, replace it with the next card. |
| `clinic_voucher` | Clinic Voucher | Gear | $2 | Before choosing a local action in a city, regain up to 2 SAN. |
| `wax_earplugs` | Wax Earplugs | Gear | $1 | Reduce the SAN loss on the revealed lieu card by 1, to a minimum of 0. |
| `bitter_rescue_salts` | Bitter Rescue Salts | Gear | $2 | If the revealed lieu card would reduce your SAN to 0, reduce the loss enough to leave you at 1 SAN. |
| `brass_fare_check` | Brass Fare Check | Gear | $1 | After a normal one-link move from a sea zone into a port, gain $2. |
| `lamplighters_book` | Lamplighter's Book | Gear | $1 | If the revealed lieu card lists a SAN loss, replace it with the next card. |
| `toolroom_hammer` | Toolroom Hammer | Weapon | $2 | Standard weapon roll. |
| `rail_yard_spike` | Rail Yard Spike | Weapon | $3 | +1 on the weapon roll against Ghouls. |
| `orchard_crook` | Orchard Crook | Weapon | $4 | +1 on the weapon roll against Fanatics. |
| `watch_captains_baton` | Watch Captain's Baton | Weapon | $4 | +1 on the weapon roll against the Professor. |
| `pier_pike` | Pier Pike | Weapon | $5 | +1 on the weapon roll against Ghouls or Fanatics. |
| `signal_pistol` | Signal Pistol | Weapon | $5 | +1 on the weapon roll against Ghouls or the Professor. |
| `stub_bore_carbine` | Stub-Bore Carbine | Weapon | $6 | +1 on the weapon roll against Fanatics or the Professor. |
| `derrick_maul` | Derrick Maul | Weapon | $4 | +2 on the weapon roll against Ghouls. |

### Unique item cards (10)

| ID | Item | Type | Magic Shop | Effect |
| --- | --- | --- | ---: | --- |
| `folding_compass` | Folding Compass | Gear | $5 | During movement, take two consecutive links of any type. |
| `tide_dial` | Tide Dial | Gear | $6 | During movement, take two or three consecutive sea links. |
| `smoked_mirror` | Smoked Mirror | Gear | $5 | Replace any revealed lieu card with the next card. |
| `unmarked_odometer` | Unmarked Odometer | Gear | $6 | During movement, take four consecutive road links. |
| `moth_silk_purse` | Moth-Silk Purse | Gear | $5 | If the revealed lieu card lists a loss of cash or SAN, gain $3 instead of applying that loss. |
| `undertow_lantern` | Undertow Lantern | Gear | $5 | When drawing a Port event, look at the top two cards, choose one to resolve, and put the other on the bottom of the draw pile. |
| `ivory_tuning_fork` | Ivory Tuning Fork | Gear | $5 | Before choosing a local action in a city, regain up to 4 SAN. |
| `reversing_thimble` | Seamstress's Reversing Thimble | Gear | $5 | When a lieu card grants a Common item, choose any Common item with a listed shop price of $4 or less as the reward instead. |
| `bell_clapper_blade` | Bell-Clapper Blade | Weapon | $7 | +2 on the weapon roll against Fanatics. |
| `blue_arc_projector` | Blue Arc Projector | Weapon | $7 | +1 on the weapon roll against Ghouls or +2 against the Professor. |

### Example turn

An investigator in Nantucket with a Ferry Voucher spends it to take two sea links, through Nantucket Sound to Edgartown. Edgartown is a port with fewer than four road links, so the investigator may draw from the shared **Port** deck or buy an available Unique item from Edgartown's **Magic Shop**. Stopping in Nantucket Sound would end their turn without a local action.
