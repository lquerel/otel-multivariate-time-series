# Benchmark Results

## Multivariate Metrics

This benchmark tests the 3 followings implementations:
* **OTEL v1** based on the current univariate metric support.
* **OTEL columnar** based on an extension of the OTEL protocol. In addition to the existing Metrics, Log and Trace object, this extension adds the generic type Event that supports a columnar representation of the data.
* **OTEL arrow** based on an extension of the Event OTEL object embedding an arrow buffer.

The internal structure of the resource object is described below.

![benchmark overview](images/benchmark_overview.svg)

> Note: OTEL columnar and OTEL arrow can be used to represent metrics, logs and traces.

For each implementation, the following operations are performed:
* **Batch creation**: creation of multiple batches of different sizes.
* **Batch processing**: execution of an equivalent processing for each batch.
* **Serialization**: serialization of the entire protobuf message for each batch.
* **Compression**: compression of the entire serialization result for each batch (LZ4).
* **Decompression**: decompression of the previously compressed batches (LZ4).
* **Deserizalization**: deserialization of the previously uncompressed batches.

![steps](images/steps.svg)

The dataset used for this benchmark can be found in the [git repository](https://github.com/lquerel/otel-multivariate-time-series). The multivariate time-series is composed of 9 labels (dimensions) and 8 metrics.

## Total time
As demonstrated by the following chart, both the OTEL columnar and OTEL arrow are much more efficient than the current OTEL implementation.