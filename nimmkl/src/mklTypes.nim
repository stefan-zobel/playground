when defined(windows):
  const
    libSuffix = ".dll"
    libPrefix = ""
elif defined(macosx):
  const
    libSuffix = ".dylib"
    libPrefix = "lib"
else:
  const
    libSuffix = ".so(||.3|.2|.1|.0)"
    libPrefix = "lib"

const
  mkl {.strdefine.} = "mkl_rt"
  libName* = libPrefix & mkl & libSuffix

{.hint: "Using MKL library with name: " & libName .}

type
  CBLAS_INDEX* = csize_t

type
  MKL_INT* = cint
  MKL_UINT* = cuint
  MKL_LONG* = clong
  MKL_INT64* = clonglong
  MKL_UINT64* = culonglong
  MKL_UINT8* = uint8
  MKL_INT8* = cchar
  MKL_INT16* = cshort
  MKL_INT32* = cint

type
  MKL_Complex8* {.importc: "MKL_Complex8", header: "mkl_types.h", bycopy.} = object
    real* {.importc: "real".}: cfloat
    imag* {.importc: "imag".}: cfloat

type
  MKL_Complex16* {.importc: "MKL_Complex16", header: "mkl_types.h", bycopy.} = object
    real* {.importc: "real".}: cdouble
    imag* {.importc: "imag".}: cdouble

type
  LapackInt* = MKL_INT
  LapackLogical* = LapackInt
  LapackComplexFloat* = MKL_Complex8
  LapackComplexDouble* = MKL_Complex16

type
  Mkl_Layout* {.size: sizeof(cint).} = enum
    MKL_ROW_MAJOR = 101,
    MKL_COL_MAJOR = 102
  Mkl_Transpose* {.size: sizeof(cint).} = enum
    MKL_NOTRANS = 111, MKL_TRANS = 112, MKL_CONJTRANS = 113
  Mkl_Uplo* {.size: sizeof(cint).} = enum
    MKL_UPPER = 121, MKL_LOWER = 122
  Mkl_Diag* {.size: sizeof(cint).} = enum
    MKL_NONUNIT = 131, MKL_UNIT = 132
  Mkl_Side* {.size: sizeof(cint).} = enum
    MKL_LEFT = 141, MKL_RIGHT = 142

type
  Cblas_Layout* {.size: sizeof(cint).} = enum
    CblasRowMajor = 101, ColMajor = 102
  Cblas_Transpose* {.size: sizeof(cint).} = enum
    CblasNoTrans = 111, CblasTrans = 112, CblasConjTrans = 113
  Cblas_Uplo* {.size: sizeof(cint).} = enum
    CblasUpper = 121, CblasLower = 122
  Cblas_Diag* {.size: sizeof(cint).} = enum
    CblasNonUnit = 131, CblasUnit = 132
  Cblas_Side* {.size: sizeof(cint).} = enum
    CblasLeft = 141, CblasRight = 142
  Cblas_Storage* {.size: sizeof(cint).} = enum
    CblasPacked = 151
  Cblas_Identifier* {.size: sizeof(cint).} = enum
    CblasAMatrix = 161, CblasBMatrix = 162
  Cblas_Offset* {.size: sizeof(cint).} = enum
    CblasRowOffset = 171, CblasColOffset = 172, CblasFixOffset = 173
  Cblas_Order* = Cblas_Layout
