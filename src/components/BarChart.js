import React from 'react'

import {
  XYPlot,
  XAxis,
  YAxis,
  VerticalGridLines,
  HorizontalGridLines,
  VerticalBarSeries,
  VerticalBarSeriesCanvas,
  LabelSeries
} from 'react-vis'


export default function BarChart({data}){

   let smaller_data = data.sort((a,b) => a.count  < b.count ? 1 : -1).slice(0,10).map((a)=> ({x:a.block_id, y: a.count}))
   console.log("smaller data ", smaller_data)
   return(
     <div>
        <XYPlot xType="ordinal" width={300} height={300} xDistance={100}>
          <XAxis />
          <YAxis />
          <VerticalBarSeries className="vertical-bar-series-example" data={smaller_data} />
        </XYPlot>
     </div>
   )
}
/*
import React from 'react';
import VegaLite from 'react-vega-lite';

const spec = {
  description: 'A simple bar chart with embedded data.',
  mark: 'bar',
  encoding: {
    y: {field: 'block_group_id', type: 'ordinal'},
    x: {field: 'count', type: 'quantitative'},
  },
};

export default function BarChart({data}) {
  return <VegaLite spec={spec} data={{values:data}} />;
}*/
