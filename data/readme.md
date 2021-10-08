Data used to for the game's initial parameters.

Sources:

- Material footprint *TODO*
    - `MATERIAL_RESOURCES_07102021004953684.csv`: <https://stats.oecd.org/Index.aspx?DataSetCode=MATERIAL_RESOURCES>, the "Material footprint per capita" data (Kilograms per capita, Thousands)
    - Material footprint "biomass, fossil fuels, metal ores and non-metal ores" (<https://unstats.un.org/sdgs/metadata/files/Metadata-08-04-01.pdf>)
    - <https://www.resourcepanel.org/sites/default/files/documents/document/media/global_material_flows_full_report_english.pdf>
        - Schandl, H., Fischer-Kowalski, M., West, J., Giljum, S., Dittrich, M., Eisenmenger, N., ... & Schaffartzik, A. (2016). Global material flows and resource productivity: assessment report for the UNEP international resource panel. United Nations Environment Programme: Paris, France.
        - Page 70 report has charts for regional material footprints from 1990-2010, broken down into subcategory (biomass, fossil fuel, metal ores, non-metallic minerals):
        - Page 99 on has individual country profiles (for 26-27 countries)
    - More complete data (1990-2017): `material_footprint/*` from <https://wesr.unep.org/indicator/index/12_2_1>
- Biodiversity *TODO*
    - `WILD_LIFE_07102021014933418.csv`: <https://stats.oecd.org/Index.aspx?DataSetCode=WILD_LIFE>, the "Threatened species as % of known species" data
    - <https://ourworldindata.org/living-planet-index>
    - <https://ourworldindata.org/extinctions>
    - <https://ourworldindata.org/biodiversity-and-wildlife>
- LCI/LCA databases
    - Indexes
        - <https://ghgprotocol.org/life-cycle-databases>
            - A list of many databases. Many are paid, or don't seem to be available anymore. I went through and pull out the ones that might be useful.
        - <https://nexus.openlca.org/databases>
            - A list of some databases
    - <https://www.lcacommons.gov>
    - <https://www.netl.doe.gov/LCA>
        - Could be useful if nothing else can be found, but in a really difficult-to-navigate format
    - <https://eplca.jrc.ec.europa.eu/LCDN/datasetList.xhtml>
        - Looks like it could be decent, but I can't load any of the actual process pages
    - <http://cpmdatabase.cpm.chalmers.se/Start.asp>
    - <https://uwaterloo.ca/canadian-raw-materials-database/>
    - <https://circularecology.com/embodied-carbon-footprint-database.html>
    - <https://www.cger.nies.go.jp/publications/report/d031/eng/page/data_file.htm>
    - <https://data.nal.usda.gov/life-cycle-assessment>
        - All at a lower level of detail than needed
    - <https://edg.epa.gov/metadata/catalog/search/resource/details.page?uuid=https://doi.org/10.23719/1517796>
        - Unfortunately this is all measured per dollar rather than per unit output (same for the other EIO/economic IO models)
    - <https://www.rivm.nl/en/life-cycle-assessment-lca/downloads>
    - Good for emissions for many processes: <https://www.ipcc-nggip.iges.or.jp/EFDB/find_ef.php?reset=>
    - EIO models
        - This is useful, though perhaps outdated, for estimating industry interdependencies: <http://www.eiolca.net/>
        - *TODO* This is more up-to-date (2012 for USEEIOv2.0): <https://www.epa.gov/land-research/us-environmentally-extended-input-output-useeio-technical-content>
            - The problem with this is it's provided as a giant excel file and suggested you use this <https://github.com/USEPA/useeior/tree/v0.4/> to access it
            - Though this might show how to interface with it using Python: <https://github.com/USEPA/USEEIO_API>
        - A similar EIO model for Canada, but potentially out of date (model is from 2002?): <https://www.ciraig.org/en/open_io_canada/documentation.html>
    - This thread is useful: <https://sustainability.stackexchange.com/questions/633/what-are-freely-available-tools-or-databases-for-lifecycle-assesment>
- Potentially useful references
    - Mineral and Energy resources: <https://stats.oecd.org/Index.aspx?DataSetCode=NAT_RES>

---

Notes:

- Basic idea is:
    - For each process get its impacts/resource requirements and mix
    - Get total impacts (CO2 emissions, etc) and subtract out of per capita amounts, so the per capita amounts only represent what isn't captured by the processes

Get water use, land use, emissions, energy use, for each non-modeled industry.
Get the total output of each non-modeled industry.

This has water per kcal: https://ourworldindata.org/water-use-stress

For energy use of water: https://www.watercalculator.org/footprint/the-water-footprint-of-energy/

So basically the only industries we model in detail are food and energy (as per the book)

We do a high-level model of other sectors but not at the process-level. We calculate per-capita estimates for these (how?) So we can estimate their macro changes as population changes. And if we have any technological developments for these we just assume they are strictly better than the old processes and so that those developments gradually spread until they saturate that entire sector.

Need to check that these aggregate values roughly match the reported totals

For energy processes we want to model scarcity issues of underlying dependencies. In the case of solar and wind, this is land use limitations, wind speed and solar irradiance limits. For others like coal, uranium, etc we need to have the current limit estimates of each resource, amount of that mineral required per kWh. As the mineral gets scarcer, we increase the amount of that mineral required to reflect increasing marginal energy requirements for extraction. For biomass we skip the actual biomass and instead just use land use/water. For the mineral/extraction-based energy sources land use is negligible (at least, mineral availability is a constraint long before land availability would be).

Of all habitable land, only 1% is for settlements, infrastructure, and other built-up land, 50% is agriculture: <https://ourworldindata.org/land-use>

Each process then has: land use, water use, energy use (ideally we split between electricity and fuel), and primary resource: e.g. coal, or soil in the case of plant calorie processes and feed in the case of animal calorie processes

Water is way more complicated because it's availability and how it replenishes etc is influenced by the climate. Not sure what to do here. Could do water stress levels (like the worldindata page) and higher temps lead to higher water stress levels and higher levels of water usage increase it too, and droughts and other shortages are a function of that and precipitation levels. Then we model crop failures in the event system; higher risk with higher water stress levels.

In the end for each region we have per capita: electricity, fuel, water, plant/animal calories.

For electricity and fuel I think we need to subtract out industry energy usage?

The other non-modeled sectors:
Mining, construction, buildings, road transportation, aviation, shipping, forestry, chemical, iron/steel, cement/concrete, waste, misc
(These should sum to all non-personal energy use)

For energy and non-energy ghgs (since energy ghgs are captured by our energy process modeling) can look at ourworldindata's emissions-by-sector
That means we need water and land use by sector. Also...ideally energy by electricity/fuel

Changes:

- Land use for each ag process type, energy process
- Model water by stress levels rather than actual quantities
    - <https://ourworldindata.org/water-use-stress#what-share-of-freshwater-resources-do-we-use>
- For non-modeled industries:
    - Assume land use to be ~0
    - Assume water use to be fixed, and that primary drivers of water scarcity will be ag and municipal use
    - Get energy, emissions for each
    - List of non-modeled industries:
        - Iron and steel
        - Road transport
        - Aviation
        - Shipping/Navigation + Rail
        - Chemical and petrochemical
        - Non-metallic minerals (i.e. concrete)
        - Buildings (Residential+Commercial)
        - Other industry (construction, non-specified, etc)
- Biodiversity should be a unitless "extinction pressure"

---

- Per capita emissions, per country, by industry: <https://ourworldindata.org/grapher/per-capita-ghg-sector?tab=table&country=~USA>

From Drew's Model:

> We assume regenerative agriculture has lower yields and therefore needs more land to produce as much food as conventional agriculture; following an estimate for organic crops, we estimate a 34% reduction in regenerative yields. See Verena Seufert et. al., ‘Comparing the Yields of Organic and Conventional Agriculture’, Nature 485, no. 7397 (2012): 229–32.

Per this chart: <https://ourworldindata.org/grapher/organic-agricultural-area>
For 2017, 4.83 billion ha for agricultural area, 18.52 million ha for organic agriculture, so organic is about 0.3% of overall agricultural land use.
If we assume 34% yield reduction in organic, as above, then the process mix of conventional ag vs organic ag is about 0.25%. This doesn't distinguish between plant/livestock agriculture, but we can just assume the mix is the same for each. Nor does it distinguish between food and non-food agriculture.

For agricultural land intensity: <https://ourworldindata.org/crop-yields>
Yield is usually calculated as tonnes/ha, but we're using kcals as our unit.
This paper: <https://iopscience.iop.org/article/10.1088/1748-9326/8/3/034015> instead calculates kcals/ha. It also has food ("delivered") vs non-food calories ("biofuels and other industrial uses"; also assuming "calories" in this paper means kcals). At the process-level we don't care about food/non-food allocation so we don't need to worry about that here. The paper says:

> From the 41 crops analyzed in this study, 9.46 × 10e15 calories available in plant form are produced by crops globally [per year], of which 55% directly feed humans. However, 36% of these produced calories go to animal feed, of which 89% is lost, such that only 4% of crop-produced calories are available to humans in the form of animal products. Another 9% of crop-produced calories are used for industrial uses and biofuels and so completely lost from the food system.

But this data is from "circa 2000" (1997-2003). From this data: <https://ourworldindata.org/grapher/total-agricultural-area-over-the-long-term>, in 2000, 4.83 billion ha was agricultural area. This translates to ~1,958,592 calories per ha.
From this data: <https://ourworldindata.org/grapher/key-crop-yields> (saved to `src/key-crop-yields.csv`), mean yields grew by 24% from 2000 to 2017:

```
import numpy as np
import pandas as pd
df = pd.read_csv('src/key-crop-yields.csv')

crops = [
   'Crops - Wheat - 15 - Yield - 5419 - hg/ha',
   'Crops - Rice, paddy - 27 - Yield - 5419 - hg/ha',
   'Crops - Maize - 56 - Yield - 5419 - hg/ha',
   'Crops - Soybeans - 236 - Yield - 5419 - hg/ha',
   'Crops - Potatoes - 116 - Yield - 5419 - hg/ha',
   'Crops - Beans, dry - 176 - Yield - 5419 - hg/ha',
   'Crops - Peas, dry - 187 - Yield - 5419 - hg/ha',
   'Crops - Cassava - 125 - Yield - 5419 - hg/ha',
   'Crops - Barley - 44 - Yield - 5419 - hg/ha',
   'Crops - Cocoa, beans - 661 - Yield - 5419 - hg/ha',
   'Crops - Bananas - 486 - Yield - 5419 - hg/ha'
]

year_2000 = df[df['Year'] == 2000]
yield_means_by_crop = [year_2000[crop].mean() for crop in crops]
yield_mean_2000 = np.mean(yield_means_by_crop)

year_2017 = df[df['Year'] == 2017]
yield_means_by_crop = [year_2017[crop].mean() for crop in crops]
yield_mean_2017 = np.mean(yield_means_by_crop)

print(yield_mean_2017/yield_mean_2000)
```

So accounting for this, we could say 2,428,654 calories per ha per year. So maybe the unit for calories should be Mcals and not kcals? This is ~2.43Mcals/ha/year or ~0.20Mcals/ha/month, so about ~5ha per Mcal.

> According to a 2011 analysis, 75% of all agricultural land (including crop and pasture land) is dedicated to animal production [11]. Livestock production is also responsible for other environmental impacts. Livestock production is estimated to be responsible for 18% of total greenhouse gas emissions [12], and animal products generally have a much higher water footprint than plant-based foods [13].

Assuming a feed conversion efficiency of 12% (from the same paper) and that livestock production's land use is dominated by feed land use, then we get something like 291,438 kcals/ha/year for livestock, or 41.6ha per Mcal.

<https://ourworldindata.org/smallholder-food-production>
Smallholders use 24% of ag land, produce 29% of crops (in kcals), some of which go into fuel and feed, and ultimately contribute to 32% of the food supply (directly consumed crops).

They tend to have higher yields (more labor intensive, less land intensive) and better biodiversity effects.

---

`Global-GHG-Emissions-by-sector-based-on-WRI-2020.xlsx`
<https://ourworldindata.org/emissions-by-sector#sector-by-sector-where-do-global-greenhouse-gas-emissions-come-from>

For 2016, total: 49.4 billion tonnes CO2eq

- Energy use: 73.2%
    - Industry: 24.2%
        - Iron & Steel: 7.2%
        - Chemical & Petrochemical: 3.6%
        - Other industry: 10.6 + 0.7 + 1 + 0.6 + 0.5 = 13.4%
    - Transport: 16.2%
        - Road transport: 11.9%
        - Aviation: 1.9%
        - Shipping: 1.7%
        - Other (Rail + Pipeline): 0.4 + 0.3 = 0.7%
    - Buildings: 17.5%
        - Commercial: 6.6%
        - Residential: 10.9%
    - Ag and Fishing: 1.7%
    - Fugitive (from energy production): 5.8%
    - Unallocated fuel combustion: 7.8%
- Industry: 5.2%
    - Cement: 3%
    - Chemicals: 2.2%
- Waste: 3.2%
    - Wastewater: 1.3%
    - Landfills: 1.9%
- Ag, forestry, land use: 18.4%
    - Livestock & manure: 5.8%
    - Ag soils: 4.1%
    - Rice cultivation: 1.3%
    - Crop burning: 3.5%
    - Deforestation: 2.2%
    - Cropland: 1.4%
    - Grassland: 0.1%

Here make the very rough assumption that energy use emissions for an industry is equivalent to the amount of energy used

Using data from: <https://www.iea.org/reports/key-world-energy-statistics-2021/final-consumption>

Total energy supply: 606EJ (168,333 TWh)
- Coal: 26.8%
- Oil: 30.9%
- Natural gas: 23.2%
- Nuclear: 5%
- Hydro: 2.5%
- Biofuels and waste: 9.4%
- Other: 2.2%
    - Geothermal, solar, wind, tide/wave/ocean, heat and other sources

Electricity generation by source: 26,936 TWh
- Nuclear: 10.4%
- Hydro: 15.7%
- Natural gas: 23.6%
- Oil: 2.8%
- Coal: 36.7%
- Non-hydro renewables and waste: 10.8%
    - Geothermal, solar, wind, tide/wave/ocean, biofuels, waste, heat and other sources

These total final energy consumption (energy use sans use by the energy sector and losses, see quote) values, in EJ (TWh in parentheses), are for 2019:

> Total  final  consumption  (TFC)  is  the  sum  of  consumption  by  the  different  end-use  sectors  and  also  includes non-energy use. Backflows from the petrochemical industry are not included in final consumption.

Total: 418EJ (116,111 TWh)
- Oil: 40.4% = 168.872
- Natural gas: 16.4% = 68.552
- Biofuels and waste: 10.4% = 43.472
- Electricity: 19.7% = 82.346
- Other: 3.6% = 15.05
    - Includes heat, solar thermal, and geothermal
- Coal: 9.5% = 39.71

Sans electricity and other: 320.606EJ
- Oil: 52.67%
- Natural gas: 21.38%
- Biofuels and waste: 13.56%
- Coal: 12.39%

So in the process of energy supply to final consumption, 188EJ was lost/used for energy production (~31% of the supply)

Coal: 40EJ (11,111 TWh)
- Iron and steel: 34.0%
- Chemical and petrochemical: 7.5%
- Non-metallic minerals: 21.7%
    - This includes cement/concrete, glass
- Other industry: 8.9%
- Residential: 6.4%
- Services agriculture and fishing: 4.2%
- Non-specified: 12.1%
    - Non-specified industry, transport and other
- Non-energy use: 5.2%

Oil: 169EJ (46,944 TWh)
- Road: 49.2%
- Aviation: 8.6%
- Industry: 7.3%
- Non-energy use: 16.7%
- Residential: 5.3%
- Navigation: 6.7%
    - Basically means marine shipping
- Rail: 0.8%
- Other: 5.4%
    - Agriculture, commercial and public services, non-specified other, pipeline, and non-specified transport

Natural gas: 68EJ (18,889 TWh)
- Industry: 37.4%
- Residential: 29.7%
- Transport: 7.3%
- Commercial and public services: 12.8%
- Non-energy use: 11.9%

Electricity: 82EJ (22,778 TWh)
- Industry: 41.9%
- Transport: 1.8%
- Commercial and public services: 21.2%
- Residential: 26.6%
- Other: 8.5%
    - Agriculture, fishing, and non-specified other

"Industry" here means:

> Industry consumption is specified by sub-sector as listed below. Energy used for transport by industry is not included here but is reported under transport. Non-energy use in industry is excluded from industry and reported separately:
> - Mining (excluding fuels) and quarrying [ISIC Divisions 07 and 08 and Group 099]
> - Construction [ISIC Divisions 41 to 43]
> - Iron and steel industry [ISIC Group 241 and Class 2431]
> - Chemical and petrochemical industry [ISIC Divisions 20 and 21] excluding petrochemical feedstocks
> - Non-ferrous metals basic industries [ISIC Group 242 and Class 2432]
> - Non-metallic minerals such as glass, ceramic, cement, etc. [ISIC Division 23]
> - Transport equipment [ISIC Divisions 29 and 30]
> - Machinery comprises fabricated metal products. machinery and equipment other than transport equipment [ISIC Divisions 25 to 28]
> - Food and tobacco [ISIC Divisions 10 to 12]
> - Paper. pulp and printing [ISIC Divisions 17 and 18]
> - Wood and wood products (other than pulp and paper) [ISIC Division 16]
> - Textile and leather [ISIC Divisions 13 to 15]
> - Non-specified (any manufacturing industry not included above) [ISIC Divisions 22. 31 and 32].

From <https://iea.blob.core.windows.net/assets/52f66a88-0b63-4ad2-94a5-29d36e864b82/KeyWorldEnergyStatistics2021.pdf>

---

Split out per-capita demands by country income classification: <https://ourworldindata.org/grapher/world-banks-income-groups?time=1990> (`world-banks-income-groups.csv`)

Basically, for non-modeled industries, we calculate a total number of "income points" with `income_weights.py`, which results in (calculated for 2016):

---


For modeled industries:

- Agriculture
    - Electricity: 82EJ * 8.5%
        - In practice this includes some other things but probably ballpark enough
    - Fuel:
        - Oil: 169EJ * 5.4%
            - Again, this actually includes some other things but probably ballpark enough
        - Coal: 40EJ * 4.2%

---

For regions we don't need different per-capita, just region income level.

Total municipal/household water withdrawals: 466283300000 m3/year, or 5.104m3 per LIC per year, or 0.425m3 per LIC per month

---

2019 electricity mix: <https://ourworldindata.org/electricity-mix>
- Oil: 3.1%
- Coal: 36.7%
- Gas: 23.5%
- Nuclear: 10.4%
- Hydropower: 15.8%
- Wind: 5.3%
- Solar: 2.7%
- Other renewables: 2.5% (geothermal, biomass, wave, tidal, but not traditional biomass)...for simplicity just allocate biomass to all of this

---

https://www.eia.gov/tools/faqs/faq.php?id=667&t=6

> The average amounts of coal, natural gas, and petroleum liquid fuels used to generate a kilowatthour (kWh) of electricity by the U.S. electric power sector in 2020 were1
> - Coal–1.13 pounds/kWh
> - Petroleum liquids–0.08 gallons/kWh
> - Natural gas–7.43 cubic feet/kWh

---

https://ourworldindata.org/smallholder-food-production
Smallholders use 24% of ag land, produce 29% of crops (in kcals), some of which go into fuel and feed, and ultimately contribute to 32% of the food supply (directly consumed crops).

So assume if they produce 29% of crops with 24% of land, about 21% greater yield than conventional ag.