import React, { useEffect, useState, useRef } from 'react';
import ReactDOM from 'react-dom';
import BarChart from './components/BarChart';
import FileLoader from './components/FileLoader'
import GeoJsonLoader from './components/GeoJsonLoader'
import PointMap from './components/PointMap'
import PolygonMap from './components/PolygonMap'
import { saveAs } from 'file-saver';


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
import { count_csv_rows } from './utils';

const startBounds = [-74.25909, 40.477399, -73.700181, 40.916178];



function App(props) {
  const [dataset, setDataset] = useState(null);
  const [polyDataset, setPolyDataset] = useState(null);
  const [blockAggs, setBlockAggs] = useState({});
  const [aggTime, setAggTime] = useState(0);

  const [log, setLog] = useState(true);
  const [pixDensity, setPixDensity] = useState(1);
  const [bounds, setBounds] = useState(startBounds);

  const reset = () => {
    setBounds(startBounds);
  };

  const pointDatasetLoaded = (dataset, columns) => {
    setDataset(dataset);
    setColumnsToAggregate(columns);
    setBounds([dataset.lng_min, dataset.lat_min, dataset.lng_max, dataset.lat_max]);
  }

  const exportGeoJSON = () => {
    const geojson = polyDataset.export_with_properties(blockAggs);
    const blob = new Blob([geojson], { type: "text/plain;charset=utf-8" })
    saveAs(blob, "aggregated_result.geojson");
  }

  const boundsChanged = (bounds) => {
    setBounds(bounds)
  }


  const onCalcIntersection = () => {
    var t0 = performance.now();
    let result = count_in_poly(polyDataset, dataset);
    var t1 = performance.now();
    setAggTime(t1 - t0)
    setBlockAggs(result.counts);
  }


  return (
    <div className={'app-layout'}>

      <div className={'header'}>
        <h1>Fast Agg</h1>
        <h3>An experiment in using wasm for GeoSpatial operations</h3>
      </div>

      <div className={'points'}>
        {dataset ?
          <div>
            <h2>{dataset.no_rows} Rows</h2>
            <PointMap onZoomIn={boundsChanged} dataset={dataset} bounds={bounds} />
          </div>
          :
          <FileLoader onLoaded={(dataset, columns) => { pointDatasetLoaded(dataset, columns) }} />
        }
      </div>

      <div className={'polygons'}>
        {polyDataset ?
          <div>
            <h2>{polyDataset.no_objects} Polygons</h2>
            <PolygonMap onZoomIn={boundsChanged} dataset={polyDataset} bounds={bounds} counts={blockAggs} />
          </div>
          : <GeoJsonLoader onLoaded={(dataset => setPolyDataset(dataset))} />
        }
      </div>

      <div className={'aggregate'}>
        {(dataset && polyDataset) &&
          <div className={'action-buttons'}>
            <button onClick={onCalcIntersection}>Aggregate</button>
            <button onClick={exportGeoJSON}>Save GeoJSON</button>
          </div>
        }
      </div>

      <div className={'footer'}>
        <p></p>
      </div>
    </div>
  );
}

let root = document.getElementById('app');
ReactDOM.render(<App />, root);
