# Copyright (c) 2022 Stefan Zobel
# Distributed under the Apache v2 License (license terms are at https://www.apache.org/licenses/LICENSE-2.0).

import mklTypes, mklTrans, mklCblas, mklLapacke
import typetraits

type TDiag* = enum
  NON_UNIT = 'N',
  UNIT = 'U'

type TEigJob* = enum
  VALUES_ONLY = 'N',
  EIGJOB_ALL = 'V'

type TNorm* = enum
  ONE = '1',
  INF = 'I'

type TRange* = enum
  RANGE_ALL = 'A',
  INDICES = 'I',
  INTERVAL = 'V'

type TSide* = enum
  LEFT = 'L',
  RIGHT = 'R'

type TSvdJob* = enum
  SVDJOB_ALL = 'A',
  NONE = 'N',
  OVERWRITE = 'O',
  PART = 'S'

type TTrans* = enum
  CONJ_TRANS = 'C',
  NO_TRANS = 'N',
  CONJ = 'R',
  TRANS = 'T'

type TUpLo* = enum
  LOWER = 'L',
  UPPER = 'U'

type TOrder* = enum
  COL_MAJOR = 'C',
  ROW_MAJOR = 'R'

proc test() =
  var x : float64
  var y: float64
  var z: cfloat
  var eigJob = TEigJob.VALUES_ONLY
  dtrmm(Cblas_Layout.CblasRowMajor, Cblas_Side.CblasLeft, Cblas_Uplo.CblasLower, Cblas_Transpose.CblasNoTrans,
  Cblas_Diag.CblasUnit, 1, 2, 1.0, addr x, 3, addr y, 4)
  simatcopy(cchar(TOrder.COL_MAJOR), cchar(TTrans.NO_TRANS), 1, 1, 1.0, addr z, 0, 0)
  echo $eigJob & " (TEigJob)"
  var aa = ($eigJob)[0]
  var bb = $eigJob
  var cc = eigJob
  echo aa
  echo aa is char
  echo bb
  echo bb is string
  echo cc
  echo cc is TEigJob
  var dd = char(cc)
  echo dd is char
  echo "dd: " & dd
  echo dd.type.name


test()

