# Open Telemetry - Multivariate time-series experiment

## Dataset
* 10000 data points represented in JSON (size uncompressed 6009735 bytes).
* Number of attributes per data point = 9
* Number of metrics per data point = 8

## Results
* The standard representation is using the current implementation of the OTEL protocol.
* The columnar representation is using a compatible evolution of the current implementation of the OTEL protocol.

It's interesting to observe that the uncompressed size of the initial json file (6009735 bytes) is:
* **4 times smaller** than the protobuf standard representation
* **6.1 times bigger** than the protobuf columnar representation

Overall the columnar representation is definitively better in every dimensions (time and space).

```
Multivariate time-series experiment (batch of 1000 data points)
Standard representation:
	uncompressed size: 2729620 bytes
	compressed size: 110580 bytes
	protobuf creation time: 0.018442229s
	protobuf serialization time: 0.014967844s
	protobuf deserialization time: 0.02135745s

Columnar representation:
	uncompressed size: 108752 bytes (25 times better)
	compressed size: 13437 bytes (8 times better)
	protobuf creation time: 0.000700462s (26.328664509994834 times better)
	protobuf serialization time: 0.000402505s (37.186728115178695 times better)
	protobuf deserialization time: 0.000880045s (24.26858853808612 times better)

Multivariate time-series experiment (batch of 2000 data points)
Standard representation:
	uncompressed size: 5459284 bytes
	compressed size: 221458 bytes
	protobuf creation time: 0.030011098s
	protobuf serialization time: 0.028587769s
	protobuf deserialization time: 0.042295567s

Columnar representation:
	uncompressed size: 216963 bytes (25 times better)
	compressed size: 25877 bytes (8 times better)
	protobuf creation time: 0.001431608s (20.963209202519124 times better)
	protobuf serialization time: 0.000758919s (37.6690648145586 times better)
	protobuf deserialization time: 0.00170514s (24.80474741076979 times better)

Multivariate time-series experiment (batch of 3000 data points)
Standard representation:
	uncompressed size: 8189004 bytes
	compressed size: 331017 bytes
	protobuf creation time: 0.046035613s
	protobuf serialization time: 0.042743589s
	protobuf deserialization time: 0.063049004s

Columnar representation:
	uncompressed size: 325186 bytes (25 times better)
	compressed size: 38199 bytes (8 times better)
	protobuf creation time: 0.001941433s (23.712182187075218 times better)
	protobuf serialization time: 0.0011618s (36.79083232914443 times better)
	protobuf deserialization time: 0.002602038s (24.23062384177326 times better)

Multivariate time-series experiment (batch of 4000 data points)
Standard representation:
	uncompressed size: 10918380 bytes
	compressed size: 440153 bytes
	protobuf creation time: 0.059017799s
	protobuf serialization time: 0.055598658s
	protobuf deserialization time: 0.085933477s

Columnar representation:
	uncompressed size: 433360 bytes (25 times better)
	compressed size: 50348 bytes (8 times better)
	protobuf creation time: 0.002717506s (21.717633374130543 times better)
	protobuf serialization time: 0.001707496s (32.56151581028594 times better)
	protobuf deserialization time: 0.00350899s (24.489518921399032 times better)

Multivariate time-series experiment (batch of 5000 data points)
Standard representation:
	uncompressed size: 13647844 bytes
	compressed size: 549455 bytes
	protobuf creation time: 0.072248193s
	protobuf serialization time: 0.070758056s
	protobuf deserialization time: 0.106452238s

Columnar representation:
	uncompressed size: 541543 bytes (25 times better)
	compressed size: 73217 bytes (7 times better)
	protobuf creation time: 0.003249329s (22.23480386258209 times better)
	protobuf serialization time: 0.002102451s (33.65503215057093 times better)
	protobuf deserialization time: 0.004441905s (23.965446807169446 times better)

Multivariate time-series experiment (batch of 6000 data points)
Standard representation:
	uncompressed size: 16377732 bytes
	compressed size: 658777 bytes
	protobuf creation time: 0.084437128s
	protobuf serialization time: 0.089135918s
	protobuf deserialization time: 0.125791744s

Columnar representation:
	uncompressed size: 649779 bytes (25 times better)
	compressed size: 87701 bytes (7 times better)
	protobuf creation time: 0.004896921s (17.24290181524268 times better)
	protobuf serialization time: 0.002650501s (33.629837528829455 times better)
	protobuf deserialization time: 0.005545655s (22.682937182352674 times better)

Multivariate time-series experiment (batch of 7000 data points)
Standard representation:
	uncompressed size: 19106940 bytes
	compressed size: 768454 bytes
	protobuf creation time: 0.103188066s
	protobuf serialization time: 0.106615174s
	protobuf deserialization time: 0.145500352s

Columnar representation:
	uncompressed size: 757930 bytes (25 times better)
	compressed size: 102040 bytes (7 times better)
	protobuf creation time: 0.004934452s (20.911757982446684 times better)
	protobuf serialization time: 0.002966476s (35.940008953384414 times better)
	protobuf deserialization time: 0.006534317s (22.267109477547535 times better)

Multivariate time-series experiment (batch of 8000 data points)
Standard representation:
	uncompressed size: 21836636 bytes
	compressed size: 878695 bytes
	protobuf creation time: 0.11761149s
	protobuf serialization time: 0.114694403s
	protobuf deserialization time: 0.176684596s

Columnar representation:
	uncompressed size: 866142 bytes (25 times better)
	compressed size: 114998 bytes (7 times better)
	protobuf creation time: 0.007062236s (16.653576855828664 times better)
	protobuf serialization time: 0.003961195s (28.954495549953993 times better)
	protobuf deserialization time: 0.007899415s (22.366795009503868 times better)

Multivariate time-series experiment (batch of 9000 data points)
Standard representation:
	uncompressed size: 24566156 bytes
	compressed size: 987544 bytes
	protobuf creation time: 0.132065103s
	protobuf serialization time: 0.139988964s
	protobuf deserialization time: 0.189726476s

Columnar representation:
	uncompressed size: 974338 bytes (25 times better)
	compressed size: 128650 bytes (7 times better)
	protobuf creation time: 0.007449753s (17.727447205296603 times better)
	protobuf serialization time: 0.003978185s (35.18915384779742 times better)
	protobuf deserialization time: 0.008340251s (22.748293306760193 times better)
```