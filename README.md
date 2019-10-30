# Wasm Geo Agg

*tl;dr*: This is a proof of concept for performing complex spatial operations (point in polygon aggregation) in the browser using WASM. On my laptop I can aggregate 13 million points to about 38,000 polygons in 21 seconds (results may vary, either faster or slower, depending on the hardware you use).

Try it live here: https://stuartlynn.github.io/wasm_geo_agg/

![screenshot](https://github.com/stuartlynn/wasm_geo_agg/blob/master/screenshots/TaxiAggregationpng.png?raw=true)

## What is this?

Wasm Geo Agg is a proof of concept to explore performing complex geospatial operations in the browser using [Rust](https://www.rust-lang.org/) and [WebAssembly](webassembly). As an initial test, we are focusing on point in polygon operations. Simply load in a CSV file 
with points and a GeoJSON file with polygons then click aggregate. 

## Give me some datasets to try! 

If you dont happen to have a bunch of geospatial datasets hanging around your computer here are a few suggestions of things to try 

### New York Street Tree Census 

- Point data: [Street trees](https://data.cityofnewyork.us/api/views/5rq2-4hqu/rows.csv?accessType=DOWNLOAD)
- Polygon data: [Census Blocks](https://data.cityofnewyork.us/api/geospatial/v2h8-6mxf?method=export&format=GeoJSON)

![New york street tree census](https://github.com/stuartlynn/wasm_geo_agg/blob/master/screenshots/StreetTrees.png?raw=true)

### New York City Taxi Pickups 

- Point data: [Taxi data for 2011 - 08](https://s3.amazonaws.com/nyc-tlc/trip+data/yellow_tripdata_2011-08.csv)
- Polygon data: [Census Blocks](https://data.cityofnewyork.us/api/geospatial/v2h8-6mxf?method=export&format=GeoJSON)

![New york street tree census](https://github.com/stuartlynn/wasm_geo_agg/blob/master/screenshots/TaxiAggregationpng.png?raw=true)

### Chicago 311 data 

- Point data: [311 complaints](https://data.cityofchicago.org/api/views/v6vf-nfxy/rows.csv?accessType=DOWNLOAD&bom=true&format=true&delimiter=%3B)
- Polgon data: [Census Blocks](https://data.cityofchicago.org/api/geospatial/mfzt-js4n?method=export&format=GeoJSON)

![Chicago 311](https://github.com/stuartlynn/wasm_geo_agg/blob/master/screenshots/Chicago311.png?raw=true)

Suggest others in an issue and I will be happy to add them!

## Why bother doing this? 

Currently, if you want to process geospatial data you can either 

1. Spend a day or two installing a bunch of really amazing tools like [GDAL](https://gdal.org/), [PostGIS](https://postgis.net/), [QGIS](https://www.qgis.org/en/site/) etc and banging your head a few times as you try to get all their versions compatible with each other ( not to mention trying to not blow up your python installation as you go)
2. Learn Python or R and use packages like [geopandas](http://geopandas.org/) 
3. Upload your data to services like [ArcGis](https://www.arcgis.com/index.html) or [CARTO](https://carto.com/) to be stored and processed in the cloud somewhere.

Options 1 or 2 let you process data locally but have a steep learning curve. As someone who has been working in the geospatial world for 4+ years, I still lose half a day each time I need to install a new geospatial stack. While using something like docker makes this a little easier, that too has a bit of a learning curve.

Option 3 means that you to some extent hand over control of your data to a third party. If the data is sensitive and needs to remain local (as is true for a lot of non-profits or research data), or if you need a service that can be guaranteed to still be around in 5-10 years, these options might not be ideal either. Another consideration is that the cloud servers that process the data on these services are often less powerful than the laptop you are using to read this article, which increasingly seems insane to me. 

So this is an experiment exploring a 4th option. To ask: what if we had a PostGIS that ran entirely in your browser? A system that uses the web to deliver sophisticated software to your computer in the form of javascript and WASM with zero installation overhead, that then processes your data locally using the powerful CPU that happens to live in your machine. 

## Is that not really slow though?

In years gone by, javascript was the only option to process anything in the browser. If this was still true then we would struggle to do mid to large scale geospatial processing in the browser. However the arrival of WebAssembly means that we can do near native processing. In this POC we use Rust and the amazing toolchain that is [wasm-pack](https://github.com/rustwasm/wasm-pack), [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [parcel](https://github.com/parcel-bundler/parcel), to stitch together a number of Rust packages that perform the calculation and then make them available to javascript. 

As mentioned, on my personal laptop, it takes about 21 seconds to aggreate 13 million points to 38,000 polygons, which is pretty mind blowing considering this is all happening in the browser.

Beyond adding the R-Tree index, I haven't done much to optimize the code so I suspect this could be made even faster. Improvements that I suspect are low hanging fruit: 

1. Making loading faster. The way I am chunking up the files to pass to Rust are not optimal I don't think. Currently it takes a while to load the data in. I am sure this could be improved
2. Rendering. The rendering code in there is super simple and was mainly just so I could verify that the data was loading and aggregating properly. The point rendering takes some inspiration from the python [datashader](https://datashader.org/) library while the polygon visualiser is just dumbly drawing paths on a 2D canvas. Both could be improved by some WebGL I suspect. Be interesting to see if this was easily integratable with [kepler.gl](https://kepler.gl/)
3. Numerical aggregations. Currently the way non spatail data is stored  and accessed is pretty dumb. I think moving to something like apache arrow could speed this up. 
4. Parallelize the code. Not sure how mature it is but WebAssembly should have multi threaded support through web workers soon. It’s possible this could be used to speed up multiple parts of the POC.

## Gross your code is ugly though...

Yeah I was very much still learning Rust and WASM while I wrote this. The plan is to refactor and make the code much more general in the next iteration. As a POC though I think it's worth putting this out there for others to see and as a call for anyone who is interested in developing geospatial tools this way to join efforts! 

Also just think how much faster better this can be when someone more competent attempts the same thing. 


## Ok ok ... I am interested ... how do I install it?

1. Install rustup:  https://rustup.rs/
2. Install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/
3. Install yarn: https://yarnpkg.com/lang/en/docs/install/#debian-stable
4. Clone this repo: `git clone https://github.com/stuartlynn/wasm_geo_agg.git`
5. Install dependencies `cd wasm_geo_agg; yarn`
6. Run `yarn start`

Let me know in the issues if you have problems getting it running

## Cool! I am sold on the future of browser delivered, natively run, geospatial tools, how can I help?

Pull requests and issues are more than welcome to this code. You can also ping me on twitter if your interested in thinking this kind of approach through more generally.


## Code structure 

The code is split into two main directories:

- _src_: Contains the javascript code which is written in React. This drives the UI and calls out to the WASM functions through wasm-bindgen. 
- _wasm_: Contains the Rust code that compiles to WebAssembly that does the heavy lifting. 



