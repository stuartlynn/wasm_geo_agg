import React, { useEffect, useState, useRef } from 'react';
import ReactDOM from 'react-dom';
import BarChart from './components/BarChart';
import FileLoader from './components/FileLoader'
import GeoJsonLoader from './components/GeoJsonLoader'
import PointMap from './components/PointMap'
import PolygonMap from './components/PolygonMap'
import { saveAs } from 'file-saver';
import Loader from 'react-loader-spinner'



import {
  agg_in_poly
} from '/wasm/Cargo.toml';
import { count_csv_rows } from './utils';

const startBounds = [-74.25909, 40.477399, -73.700181, 40.916178];


function App(props) {
  const [dataset, setDataset] = useState(null);
  const [polyDataset, setPolyDataset] = useState(null);
  const [blockAggs, setBlockAggs] = useState({});
  const [aggTime, setAggTime] = useState(0);
  const [aggregated, setAggregated] = useState(false);

  const [columnsToAggregate, setColumnsToAggregate] = useState([])
  const [bounds, setBounds] = useState(startBounds);

  const reset = (e) => {
    e.preventDefault();
    setBounds([dataset.lng_min, dataset.lat_min, dataset.lng_max, dataset.lat_max]);
  };

  const pointDatasetLoaded = ({ dataset, columns }) => {
    setDataset(dataset);
    setColumnsToAggregate(Object.keys(columns));
    console.log('dataset bounds ', dataset.lng_min, dataset.lat_min, dataset.lng_max, dataset_lat_max);
    setBounds([dataset.lng_min, dataset.lat_min, dataset.lng_max, dataset.lat_max]);
  }

  const exportGeoJSON = () => {
    const geojson = polyDataset.export_geo_json();
    const blob = new Blob([geojson], { type: "text/plain;charset=utf-8" })
    saveAs(blob, "aggregated_result.geojson");
  }

  const boundsChanged = (bounds) => {
    setBounds(bounds)
  }

  const onCalcIntersection = () => {
    var t0 = performance.now();
    let result = agg_in_poly(polyDataset, dataset);
    let r = result.counts
    var t1 = performance.now();
    setAggTime((t1 - t0) / 1000)

    Object.keys(r).forEach(regionID => {
      Object.keys(r[regionID]).forEach(col => {
        if (col !== 'count') {
          result.counts[regionID][`${col}_avg`] = r[regionID][col] / r[regionID]['count']
        }
      })
    })
    r = result.counts

    let formated_result = {}

    Object.keys(r).forEach(regionID => {
      Object.keys(r[regionID]).forEach(col => {
        if (formated_result[col]) {
          formated_result[col][regionID] = r[regionID][col]
        }
        else {
          formated_result[col] = { regionID: r[regionID][col] }
        }
      })
    })

    Object.keys(formated_result).forEach(k => {
      polyDataset.assign(k, formated_result[k]);
    })
    console.log('result is ', result)
    setAggregated(true)
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
            <PointMap onZoomIn={boundsChanged} dataset={dataset} bounds={bounds} />
            <p>Click map to zoom. <a style={{ color: '#D5A583', textDecoration: 'none' }} href='' onClick={reset}>Reset view</a></p>
          </div>
          :
          <FileLoader onLoaded={(dataset, columns) => { pointDatasetLoaded(dataset, columns) }} />
        }
      </div>

      <div className={'polygons'}>
        {polyDataset ?
          <div>
            <PolygonMap columns={columnsToAggregate} onZoomIn={boundsChanged} dataset={polyDataset} bounds={bounds} counts={blockAggs} />
            <p>Click map to zoom. <a style={{ color: '#D5A583', textDecoration: 'none' }} href='' onClick={reset}>Reset view</a></p>
          </div>
          :
          <GeoJsonLoader onLoaded={(dataset => setPolyDataset(dataset))} />
        }
      </div>

      <div className={'aggregate'}>

        {(dataset && polyDataset) ?
          <div className={'action-buttons'}>
            {aggregated ?
              <div>
                <h2>Aggregation took {aggTime.toPrecision(3)}s</h2>
                <button onClick={exportGeoJSON}>Save GeoJSON</button>
              </div>
              :
              <button onClick={onCalcIntersection}>Aggregate</button>
            }

          </div>
          :
          <p>Select a csv containing latitude and logitude point data, and a geojson containing polygon data to aggregate to.</p>
        }
      </div>

      <div className={'footer'}>
        <span>Code</span>
        <span>Twitter</span>
        <span>About</span>
        <span>Me (Stuart Lynn)</span>
      </div>
    </div>
  );
}

let root = document.getElementById('app');
ReactDOM.render(<App />, root);
