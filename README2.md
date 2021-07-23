# Metrics Benchmark Results (v2)

This benchmark tests the 3 followings implementations:
* The **OTEL v1**  is based on the current univariate metric support of the OTEL protocol.
* The **OTEL columnar** is based on an extension of the OTEL protocol. In addition to the existing Metrics, Log and Trace object, this extension adds the generic type Event that supports a columnar representation of the data.
* The **OTEL arrow** is based on an extension of the Event OTEL object embedding an arrow buffer.

Note: OTEL columnar and OTEL arrow can be used to represent metrics, logs and traces.

![benchmark overview](/images/benchmark_overview.png)

For each implementation, the following operations are performed:
* **Batch creation**: creation of multiple batches of different sizes.
* **Batch processing**: execution of an equivalent processing for each batch.
* **Serialization**: serialization of the entire protobuf message for each batch.
* **Compression**: compression of the entire serialization result for each batch (LZ4).
* **Decompression**: decompression of the previously compressed batches (LZ4).
* **Deserizalization**: deserialization of the previously uncompressed batches.

The dataset used for this benchmark can be found in the [git repository](https://github.com/lquerel/otel-multivariate-time-series). The multivariate time-series is composed of 9 labels (dimensions) and 8 metrics.

## Total time
As demonstrated by the following chart, both the OTEL columnar and OTEL arrow are much more efficient than the current OTEL implementation.