import React from 'react'

export default function ProgressBar({ total, value, style }) {
    return (
        <div className='progress-bar-outer' style={style}>
            <div className={'progress-bar-inner'} style={{ width: `${value * 100.0 / total}%` }}>
            </div>
            <p className='progress-bar-pc'>{(value * 100 / total).toPrecision(3)} %</p>
        </div>
    )
}