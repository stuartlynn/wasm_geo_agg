import React, {useEffect, useState, useRef} from 'react';
import ReactDOM from 'react-dom';
import BarChart from './components/BarChart';

import {
  add,
  parse_csv,
  Dataset,
  load_csv,
  load_geojson,
  PolygonDataset,
  count_in_poly,
  test_map
} from '/wasm/Cargo.toml';
import {count_csv_rows} from './utils';

const startBounds = [-74.25909, 40.477399, -73.700181, 40.916178];

function App(props) {
  const [dataset, setDataset] = useState(null);
  const [polyDataset, setPolyDataset] = useState(null);
  const [blockAggs, setBlockAggs] = useState({});
  const [log, setLog] = useState(true);
  const [pixDensity, setPixDensity] = useState(1);

  const datasetRef = useRef(null);
  const polyDatasetRef = useRef(null);

  const [bounds, setBounds] = useState(startBounds);

  useEffect(()=>{
    console.log("trying hash map")
    test_map(2.3,{test: 2.0})
  },[])

  const [rows, setRows] = useState(0);
  useEffect(() => {
    fetch('/2015StreetTreesCensus_TREES.csv')
      .then(r => r.text())
      .then(r => {
        let t0 = performance.now();
        let dataset = load_csv(r);
        let t1 = performance.now();
        console.log('loading csv tool ', (t1 - t0) / 1000.0, ' ms');
        setDataset(dataset);
        datasetRef.current = dataset;
      });

    fetch('/2010CensusBlocks.geojson')
      .then(r => r.text())
      .then(r => {
        var t0 = performance.now();
        let geo_dataset = load_geojson(r);
        var t1 = performance.now();
        polyDatasetRef.current = geo_dataset;
        setPolyDataset(geo_dataset);
      });
  }, []);

  const reset = () => {
    setBounds(startBounds);
  };


  const onCalcIntersection = ()=>{
      console.log('Starting point in poly');
      var t0 = performance.now();
      let result = count_in_poly(polyDataset, dataset);
      var t1 = performance.now();
      console.log('Took ' + (t1 - t0) / 1000.0 + ' s.');
      console.log('got result ', result);
      setBlockAggs(result.counts);
      console.log('set block aggs ', result);
  }


  useEffect(()=>{
        if(polyDataset){
            console.log('blockaggs', blockAggs)
            polyDataset.display_with_counts(
                    blockAggs,
                    bounds[0],
                    bounds[1],
                    bounds[2],
                    bounds[3],
                )
        }
    
  },[blockAggs, polyDataset])

  useEffect(() => {
    if(polyDataset){
        polyDataset.display_with_counts(
                blockAggs,
                bounds[0],
                bounds[1],
                bounds[2],
                bounds[3],
            )
    }
    if (dataset) {
      dataset.render_to_canvas(
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
        1000,
        log,
        pixDensity,
      );
    }
  }, [polyDataset, dataset, log, bounds, pixDensity]);

  const zoomIn = e => {
    console.log(e.pageX, e.pageY);
    const x = (e.pageX * (bounds[2] - bounds[0])) / 1200.0 + bounds[0];
    const y = (e.pageX * (bounds[3] - bounds[1])) / 1200.0 + bounds[1];
    const xrange = (bounds[2] - bounds[0]) * 0.75;
    const yrange = (bounds[3] - bounds[1]) * 0.75;

    setBounds([
      x - xrange / 2.0,
      y - yrange / 2.0,
      x + xrange / 2.0,
      y + yrange / 2.0,
    ]);
  };

  return (
    <div>
      <h1>Hello WorldIt's me mario.Test!! </h1>
      {dataset && <h2>Rows {dataset.no_rows}</h2>}
      <span>
        Log:
        <input
          type="checkbox"
          checked={log}
          onChange={() => {
            setLog(!log);
          }}
        />
      </span>
      <span>
        {pixDensity}
        <input
          type="range"
          value={pixDensity}
          min={1}
          max={10}
          step={1}
          onChange={e => setPixDensity(e.target.value)}
        />
      </span>
      <button onClick={reset}>Reset</button>
      { (dataset && polyDataset) &&  
         <button onClick={onCalcIntersection}>Aggregate</button>
      }
      <canvas
        id="canvas"
        width={1000}
        height={1000}
        style={{width: 600, height: 600, border: '1px solid black'}}
        onClick={zoomIn}
      />
      <canvas
        id="canvas2"
        width={1000}
        height={1000}
        style={{width: 600, height: 600, border: '1px solid black'}}
        onClick={zoomIn}
      />
      {blockAggs && (
        <BarChart
          data={Object.keys(blockAggs).map(key => ({
            block_id: key,
            count: blockAggs[key],
          }))}
        />
   )}
    </div>
  );
}

let root = document.getElementById('app');
ReactDOM.render(<App />, root);
