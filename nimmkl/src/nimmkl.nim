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

type
  MatrixLayout* = enum
    ROW_MAJOR_ORDER = LAPACK_ROW_MAJOR,
    COL_MAJOR_ORDER = LAPACK_COL_MAJOR

proc dgbsv*(n: int; kl: int; ku: int;
           nrhs: int; ab: seq[float64]; ldab: int; ipiv: ptr cint;
           b: seq[float64]; ldb: int): int =
  let xx = if ab.len == 0: nil else: unsafeAddr(ab[0])
  let yy = if b.len == 0: nil else: unsafeAddr(b[0])
  dgbsv(COL_MAJOR_ORDER.cint, n.cint, kl.cint, ku.cint, nrhs.cint, xx, ldab.cint, ipiv, yy, ldb.cint)

proc test() =
  var x : float64
  var y : float64
  var z : cfloat
  var ab : seq[float64] = @[float64 1, 2, 3]
  var b : seq[float64] = @[float64 4, 5, 6]
  var ipiv : cint = 1

  dtrmm(Cblas_Layout.CblasRowMajor, Cblas_Side.CblasLeft, Cblas_Uplo.CblasLower, Cblas_Transpose.CblasNoTrans,
  Cblas_Diag.CblasUnit, 1, 2, 1.0, addr x, 3, addr y, 4)
  simatcopy(cchar(TOrder.COL_MAJOR), cchar(TTrans.NO_TRANS), 1, 1, 1.0, addr z, 0, 0)
  let res = dgbsv(1, 1, 1, 1, ab, 0, addr ipiv, b, 1)
  echo res

  var eigJob = TEigJob.VALUES_ONLY
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

