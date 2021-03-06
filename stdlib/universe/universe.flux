package universe

import "system"
import "date"
import "math"
import "strings"
import "regexp"

// now is a function option whose default behaviour is to return the current system time
option now = system.time

// Booleans
builtin true
builtin false

// Transformation functions
builtin chandeMomentumOscillator
builtin columns
builtin count
builtin covariance
builtin cumulativeSum
builtin derivative
builtin difference
builtin distinct
builtin drop
builtin duplicate
builtin elapsed
builtin exponentialMovingAverage
builtin fill
builtin filter
builtin first
builtin group
builtin histogram
builtin histogramQuantile
builtin holtWinters
builtin hourSelection
builtin integral
builtin join
builtin kaufmansAMA
builtin keep
builtin keyValues
builtin keys
builtin last
builtin limit
builtin map
builtin max
builtin mean
builtin min
builtin mode
builtin movingAverage
builtin quantile
builtin pivot
builtin range
builtin reduce
builtin relativeStrengthIndex
builtin rename
builtin sample
builtin set
builtin tail
builtin timeShift
builtin skew
builtin spread
builtin sort
builtin stateTracking
builtin stddev
builtin sum
builtin tripleExponentialDerivative
builtin union
builtin unique
builtin window
builtin yield

// stream/table index functions
builtin tableFind
builtin getColumn
builtin getRecord

// type conversion functions
builtin bool
builtin bytes
builtin duration
builtin float
builtin int
builtin string
builtin time
builtin uint

// contains function
builtin contains

// other builtins
builtin inf
builtin length // length function for arrays
builtin linearBins
builtin logarithmicBins
builtin sleep // sleep is the identity function with the side effect of delaying execution by a specified duration

// covariance function with automatic join
cov = (x,y,on,pearsonr=false) =>
    join(
        tables:{x:x, y:y},
        on:on,
    )
    |> covariance(pearsonr:pearsonr, columns:["_value_x","_value_y"])

pearsonr = (x,y,on) => cov(x:x, y:y, on:on, pearsonr:true)

// AggregateWindow applies an aggregate function to fixed windows of time.
// The procedure is to window the data, perform an aggregate operation,
// and then undo the windowing to produce an output table for every input table.
aggregateWindow = (every, fn, column="_value", timeSrc="_stop",timeDst="_time", createEmpty=true, tables=<-) =>
    tables
        |> window(every:every, createEmpty: createEmpty)
        |> fn(column:column)
        |> duplicate(column:timeSrc,as:timeDst)
        |> window(every:inf, timeColumn:timeDst)

// AggregateWindow applies an aggregate function to fixed windows of time.
// The procedure is to window the data, perform an aggregate operation,
// and then undo the windowing to produce an output table for every input table.
agg = (every, fn, column="_value", timeSrc="_start",timeDst="_time", createEmpty=true, tables=<-) =>
    tables
        |> window(every:every, createEmpty: createEmpty)
        |> fn(column:column)
        |> duplicate(column:timeSrc,as:timeDst)
        |> window(every:inf, timeColumn:timeDst)

// Increase returns the total non-negative difference between values in a table.
// A main usage case is tracking changes in counter values which may wrap over time when they hit
// a threshold or are reset. In the case of a wrap/reset,
// we can assume that the absolute delta between two points will be at least their non-negative difference.
increase = (tables=<-, columns=["_value"]) =>
    tables
        |> difference(nonNegative: true, columns:columns)
        |> cumulativeSum(columns: columns)

// median returns the 50th percentile.
median = (method="estimate_tdigest", compression=0.0, column="_value", tables=<-) =>
    tables
        |> quantile(q:0.5, method: method, compression: compression, column: column)

// stateCount computes the number of consecutive records in a given state.
// The state is defined via the function fn. For each consecutive point for
// which the expression evaluates as true, the state count will be incremented
// When a point evaluates as false, the state count is reset.
//
// The state count will be added as an additional column to each record. If the
// expression evaluates as false, the value will be -1. If the expression
// generates an error during evaluation, the point is discarded, and does not
// affect the state count.
stateCount = (fn, column="stateCount", tables=<-) =>
    tables
        |> stateTracking(countColumn:column, fn:fn)

// stateDuration computes the duration of a given state.
// The state is defined via the function fn. For each consecutive point for
// which the expression evaluates as true, the state duration will be
// incremented by the duration between points. When a point evaluates as false,
// the state duration is reset.
//
// The state duration will be added as an additional column to each record. If the
// expression evaluates as false, the value will be -1. If the expression
// generates an error during evaluation, the point is discarded, and does not
// affect the state duration.
//
// Note that as the first point in the given state has no previous point, its
// state duration will be 0.
//
// The duration is represented as an integer in the units specified.
stateDuration = (fn, column="stateDuration", timeColumn="_time", unit=1s, tables=<-) =>
    tables
        |> stateTracking(durationColumn:column, timeColumn:timeColumn, fn:fn, durationUnit:unit)

// _sortLimit is a helper function, which sorts and limits a table.
_sortLimit = (n, desc, columns=["_value"], tables=<-) =>
    tables
        |> sort(columns:columns, desc:desc)
        |> limit(n:n)

// top sorts a table by columns and keeps only the top n records.
top = (n, columns=["_value"], tables=<-) =>
    tables
        |> _sortLimit(n:n, columns:columns, desc:true)

// top sorts a table by columns and keeps only the bottom n records.
bottom = (n, columns=["_value"], tables=<-) =>
    tables
        |> _sortLimit(n:n, columns:columns, desc:false)

// _highestOrLowest is a helper function, which reduces all groups into a single group by specific tags and a reducer function,
// then it selects the highest or lowest records based on the column and the _sortLimit function.
// The default reducer assumes no reducing needs to be performed.
_highestOrLowest = (n, _sortLimit, reducer, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> group(columns:groupColumns)
        |> reducer()
        |> group(columns:[])
        |> _sortLimit(n:n, columns:[column])

// highestMax returns the top N records from all groups using the maximum of each group.
highestMax = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                // TODO(nathanielc): Once max/min support selecting based on multiple columns change this to pass all columns.
                reducer: (tables=<-) => tables |> max(column:column),
                _sortLimit: top,
            )

// highestAverage returns the top N records from all groups using the average of each group.
highestAverage = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                reducer: (tables=<-) => tables |> mean(column:column),
                _sortLimit: top,
            )

// highestCurrent returns the top N records from all groups using the last value of each group.
highestCurrent = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                reducer: (tables=<-) => tables |> last(column:column),
                _sortLimit: top,
            )

// lowestMin returns the bottom N records from all groups using the minimum of each group.
lowestMin = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                // TODO(nathanielc): Once max/min support selecting based on multiple columns change this to pass all columns.
                reducer: (tables=<-) => tables |> min(column:column),
                _sortLimit: bottom,
            )

// lowestAverage returns the bottom N records from all groups using the average of each group.
lowestAverage = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                reducer: (tables=<-) => tables |> mean(column:column),
                _sortLimit: bottom,
            )

// lowestCurrent returns the bottom N records from all groups using the last value of each group.
lowestCurrent = (n, column="_value", groupColumns=[], tables=<-) =>
    tables
        |> _highestOrLowest(
                n:n,
                column:column,
                groupColumns:groupColumns,
                reducer: (tables=<-) => tables |> last(column:column),
                _sortLimit: bottom,
            )

// timedMovingAverage constructs a simple moving average over windows of 'period' duration
// eg: A 5 year moving average would be called as such:
//    movingAverage(1y, 5y)
timedMovingAverage = (every, period, column="_value", tables=<-) =>
    tables
        |> window(every: every, period: period)
        |> mean(column:column)
        |> duplicate(column: "_stop", as: "_time")
        |> window(every: inf)

// Double Exponential Moving Average computes the double exponential moving averages of the `_value` column.
// eg: A 5 point double exponential moving average would be called as such:
// from(bucket: "telegraf/autogen"):
//    |> range(start: -7d)
//    |> doubleEMA(n: 5)
doubleEMA = (n, tables=<-) =>
    tables
          |> exponentialMovingAverage(n:n)
          |> duplicate(column:"_value", as:"__ema")
          |> exponentialMovingAverage(n:n)
          |> map(fn: (r) => ({r with _value: 2.0*r.__ema - r._value}))
          |> drop(columns: ["__ema"])


// Triple Exponential Moving Average computes the triple exponential moving averages of the `_value` column.
// eg: A 5 point triple exponential moving average would be called as such:
// from(bucket: "telegraf/autogen"):
//    |> range(start: -7d)
//    |> tripleEMA(n: 5)
tripleEMA = (n, tables=<-) =>
	tables
		|> exponentialMovingAverage(n:n)
		|> duplicate(column:"_value", as:"__ema1")
		|> exponentialMovingAverage(n:n)
		|> duplicate(column:"_value", as:"__ema2")
		|> exponentialMovingAverage(n:n)
		|> map(fn: (r) => ({r with _value: 3.0*r.__ema1 - 3.0*r.__ema2 + r._value}))
		|> drop(columns: ["__ema1", "__ema2"])

// truncateTimeColumn takes in a time column t and a Duration unit and truncates each value of t to the given unit via map
// Change from _time to timeColumn once Flux Issue 1122 is resolved
truncateTimeColumn = (timeColumn="_time", unit, tables=<-) =>
    tables
        |> map(fn:(r) => ({r with _time: date.truncate(t: r._time, unit: unit)}))

// kaufmansER computes Kaufman's Efficiency Ratios of the `_value` column
kaufmansER = (n, tables=<-) =>
    tables
        |> chandeMomentumOscillator(n: n)
        |> map(fn:(r) => ({r with _value: (math.abs(x: r._value)/100.0)}))

toString   = (tables=<-) => tables |> map(fn:(r) => ({r with _value: string(v:r._value)}))
toInt      = (tables=<-) => tables |> map(fn:(r) => ({r with _value: int(v:r._value)}))
toUInt     = (tables=<-) => tables |> map(fn:(r) => ({r with _value: uint(v:r._value)}))
toFloat    = (tables=<-) => tables |> map(fn:(r) => ({r with _value: float(v:r._value)}))
toBool     = (tables=<-) => tables |> map(fn:(r) => ({r with _value: bool(v:r._value)}))
toTime     = (tables=<-) => tables |> map(fn:(r) => ({r with _value: time(v:r._value)}))
