//! Compatible wrapper functions for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.
//!
//! All functions are resolved when the shared library is loaded; a runtime
//! panic occurs if a function is not found.

use super::*;
use core::ffi::{c_int, c_void};

#[doc = " @brief Creates a new cuEST handle and its associated resources from a parameter structure.\n @ingroup Context\n\n Ownership of handles or pointers passed into the structure is NOT transferred.\n Default parameters will construct a generic GPU context.\n\n Requirements:\n   - A cudaStream alone may be provided, all others may be left NULL.\n   - If cuBLAS or cuSolver handles are specified, a cudaStream must also be specified.\n   - When cuBLAS or cuSolver handles are specified, these handles must be set\n     to operate on the provided cudaStream. (i.e. by calling \\c cublasSetStream\n     or \\c cusolverDnSetStream with the provided cudaStream);\n\n @param[in]  parameters         Parameter structure handle (must not be NULL).\n @param[out] handle             Pointer to opaque cuEST handle for output.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_NULL_POINTER if \\c parameters or \\c handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c parameters is not of the \\c cuestHandleParameters_t type,\n   - \\c CUEST_STATUS_UNSUPPORTED_ARCHITECTURE if the GPU architecture is not supported by cuEST,\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a handle parameter value is invalid (e.g. an invalid JIT compile-thread count or an unsafe JIT cache directory),\n   - \\c CUEST_STATUS_HOME_NOT_FOUND if no JIT cache directory was specified and the HOME environment variable is unset or empty,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestCreate(
    parameters: cuestHandleParameters_t,
    handle: *mut cuestHandle_t,
) -> cuestStatus_t {
    dyload_lib().cuestCreate.expect("cuEST function `cuestCreate` not found in the loaded library")(
        parameters, handle,
    )
}

#[doc = " @brief Destroys a cuEST handle and all associated resources.\n @ingroup Context\n\n Frees memory for GPU resources, quadrature data, solid harmonics coefficients, and rys table.\n\n @param[in] handle  Opaque cuEST handle to destroy (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_NULL_POINTER if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDestroy(handle: cuestHandle_t) -> cuestStatus_t {
    dyload_lib()
        .cuestDestroy
        .expect("cuEST function `cuestDestroy` not found in the loaded library")(handle)
}

#[doc = " @brief Sets the math mode used when cuEST functions are called with this handle.\n @ingroup Context\n\n @param[in,out] handle  Opaque cuEST handle (must not be NULL).\n @param[in] mode    Math mode enum.\n\n - \\c CUEST_DEFAULT_MATH_MODE cuEST will use algorithms that are determined heuristically\n      to maximize performance. Some results may use mixed precision arithmetic.\n - \\c CUEST_NATIVE_FP64_MATH_MODE cuEST will use FP64 arithmetic everywhere.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c cuestMathMode_t is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestSetMathMode(handle: cuestHandle_t, mode: cuestMathMode_t) -> cuestStatus_t {
    dyload_lib()
        .cuestSetMathMode
        .expect("cuEST function `cuestSetMathMode` not found in the loaded library")(
        handle, mode
    )
}

#[doc = " @brief Gets the math mode associated with this handle.\n @ingroup Context\n\n @param[in]  handle       Opaque cuEST handle (must not be NULL).\n @param[out] mode         Math mode enum (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_NULL_POINTER if mode is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestGetMathMode(handle: cuestHandle_t, mode: *mut cuestMathMode_t) -> cuestStatus_t {
    dyload_lib()
        .cuestGetMathMode
        .expect("cuEST function `cuestGetMathMode` not found in the loaded library")(
        handle, mode
    )
}

#[doc = " @brief Sets the compute capability used in heuristics to determine the most performant code path.\n @ingroup Context\n\n @param[in,out] handle               Opaque cuEST handle (must not be NULL).\n @param[in] targetMajorVersion   Compute capability target major version.\n @param[in] targetMinorVersion   Compute capability target minor version.\n\n Notes:\n   - Manually setting the compute capability target is not recommended for production calculations.\n     This option is intended only to compare numerical performance of the same algorithm on different\n     architectures.\n   - Not all algorithms may be available on all architectures. Changing the compute capability\n     target may result in exceptions during subsequent calls to cuEST.\n   - The compute capability target only affects heuristics. It does not affect kernel execution\n     or JIT compilation.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestSetComputeCapabilityTarget(
    handle: cuestHandle_t,
    targetMajorVersion: u32,
    targetMinorVersion: u32,
) -> cuestStatus_t {
    dyload_lib()
        .cuestSetComputeCapabilityTarget
        .expect("cuEST function `cuestSetComputeCapabilityTarget` not found in the loaded library")(
        handle,
        targetMajorVersion,
        targetMinorVersion,
    )
}

#[doc = " @brief Gets the compute capability used in heuristics to determine the most performant code path.\n @ingroup Context\n\n @param[in]  handle              Opaque cuEST handle (must not be NULL).\n @param[out] targetMajorVersion  Compute capability target major version (must not be NULL).\n @param[out] targetMinorVersion  Compute capability target minor version (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_NULL_POINTER if targetMajorVersion or targetMinorVersion is NULL,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestGetComputeCapabilityTarget(
    handle: cuestHandle_t,
    targetMajorVersion: *mut u32,
    targetMinorVersion: *mut u32,
) -> cuestStatus_t {
    dyload_lib()
        .cuestGetComputeCapabilityTarget
        .expect("cuEST function `cuestGetComputeCapabilityTarget` not found in the loaded library")(
        handle,
        targetMajorVersion,
        targetMinorVersion,
    )
}

#[doc = " @brief Gets the major version of the cuEST library.\n @ingroup Context\n\n @param[in]  handle              Opaque cuEST handle (must not be NULL).\n @param[out] majorVersion        Major version of the cuEST library (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_NULL_POINTER if majorVersion is NULL,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestGetMajorVersion(handle: cuestHandle_t, majorVersion: *mut u32) -> cuestStatus_t {
    dyload_lib()
        .cuestGetMajorVersion
        .expect("cuEST function `cuestGetMajorVersion` not found in the loaded library")(
        handle,
        majorVersion,
    )
}

#[doc = " @brief Gets the minor version of the cuEST library.\n @ingroup Context\n\n @param[in]  handle              Opaque cuEST handle (must not be NULL).\n @param[out] minorVersion        Minor version of the cuEST library (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_NULL_POINTER if minorVersion is NULL,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestGetMinorVersion(handle: cuestHandle_t, minorVersion: *mut u32) -> cuestStatus_t {
    dyload_lib()
        .cuestGetMinorVersion
        .expect("cuEST function `cuestGetMinorVersion` not found in the loaded library")(
        handle,
        minorVersion,
    )
}

#[doc = " @brief Gets the patch version of the cuEST library.\n @ingroup Context\n\n @param[in]  handle              Opaque cuEST handle (must not be NULL).\n @param[out] patchVersion        Patch version of the cuEST library (must not be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success,\n   - \\c CUEST_STATUS_INVALID_HANDLE if handle is NULL,\n   - \\c CUEST_STATUS_INVALID_TYPE if handle is not of the \\c cuestHandle_t type,\n   - \\c CUEST_STATUS_NULL_POINTER if patchVersion is NULL,\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestGetPatchVersion(handle: cuestHandle_t, patchVersion: *mut u32) -> cuestStatus_t {
    dyload_lib()
        .cuestGetPatchVersion
        .expect("cuEST function `cuestGetPatchVersion` not found in the loaded library")(
        handle,
        patchVersion,
    )
}

#[doc = " @brief Create an AO shell object with specified parameters.\n @ingroup Basis\n\n @param[in]  handle          cuEST handle. Must not be NULL.\n @param[in]  isPure          Whether the shell uses pure (spherical) functions (any nonzero value is interpreted as true).\n @param[in]  L               Angular momentum quantum number (s=0, p=1, d=2, etc).\n @param[in]  numPrimitive    Number of Gaussian primitives; must be greater than zero.\n @param[in]  exponents       Pointer containing primitive exponents (size: \\c numPrimitive) on the CPU. Must not be NULL.\n @param[in]  coefficients    Pointer containing contraction coefficients (size: \\c numPrimitive) on the CPU. Must not be NULL.\n @param[in]  parameters      Handle to \\c cuestAOShellParameters_t. Must not be NULL.\n @param[out] outShell        Pointer to output shell object; receives opaque handle on success. Must not be NULL.\n\n The contraction \\c coefficients passed to this function will be used verbatim. It is the user's responsibility\n to normalize the shells as desired. For more information see: \\verbatim embed:rst\n:ref:`cuest_primitive_normalization`\n\\endverbatim\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle or parameters are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numPrimitive is zero;\n   - \\c CUEST_STATUS_EXCEPTION for unexpected error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR for non-std exceptions."]
pub unsafe fn cuestAOShellCreate(
    handle: cuestHandle_t,
    isPure: i32,
    L: u64,
    numPrimitive: u64,
    exponents: *const f64,
    coefficients: *const f64,
    parameters: cuestAOShellParameters_t,
    outShell: *mut cuestAOShell_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAOShellCreate
        .expect("cuEST function `cuestAOShellCreate` not found in the loaded library")(
        handle,
        isPure,
        L,
        numPrimitive,
        exponents,
        coefficients,
        parameters,
        outShell,
    )
}

#[doc = " @brief Destroy/free an AO shell object created with \\c cuestAOShellCreate.\n @ingroup Basis\n\n Frees all memory associated with the AO shell. After destruction, the shell handle must not be used.\n\n @param[in] shell   Opaque AO shell handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c shell is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the \\c shell handle is not of the \\c cuestAOShell_t type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAOShellDestroy(shell: cuestAOShell_t) -> cuestStatus_t {
    dyload_lib()
        .cuestAOShellDestroy
        .expect("cuEST function `cuestAOShellDestroy` not found in the loaded library")(shell)
}

#[doc = " @brief Create an atomic orbital basis set object (AO basis).\n @ingroup Basis\n\n The AO basis comprises multiple AO shells, grouped by atomic center.\n - All buffer pointers must be valid (not NULL).\n - Each shell, and each array entry, must be valid.\n - The persistent and temporary workspaces must be provided and preallocated.\n - The output handle is only valid after successful creation.\n\n \\verbatim embed:rst\n.. Note:: After creating the `cuestAOBasis_t` basis handle, the array of `cuestAOShell_t` shell handles are no longer necessary and may be destroyed and any user provided input data to those shell handles may be freed.\n\\endverbatim\n\n \\verbatim embed:rst\n.. Important::\nBasis sets with \"holes\" in the shell angular momentum are not supported. That is, if a\nbasis function center has a p function (L=1), it must also have an s function (L=0). If it has\na d function (L=2), it must also have s and p functions present.\nA basis that violates this requirement is rejected with ``CUEST_STATUS_EXCEPTION``.\n\\endverbatim\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  numAtoms              Total number of distinct atoms/centers; must be greater than zero.\n @param[in]  numShellsPerAtom      Array specifying number of shells per center (size: \\c numAtoms); each must be greater than zero. CPU array, must not be NULL.\n @param[in]  shells                Flat array of shell handles (\\c cuestAOShell_t), length is sum of \\c numShellsPerAtom.\n                                   CPU array, the array must not be NULL and none of the entries may be NULL.\n @param[in]  parameters            Handle to \\c cuestAOBasisParameters_t. Must not be NULL.\n @param[in]  persistentWorkspace   Pointer to preallocated workspace for persistent buffers. Must not be NULL.\n @param[in]  temporaryWorkspace    Pointer to preallocated workspace for temporary buffers. Must not be NULL.\n @param[out] outBasis              Pointer for output basis handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer parameter is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle, any shell handle or parameters are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms == 0 or any \\c numShellsPerAtom[i]==0;\n   - \\c CUEST_STATUS_UNSUPPORTED_ARGUMENT if the AO basis mixes pure (solid-harmonic) and Cartesian shells;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAOBasisCreate(
    handle: cuestHandle_t,
    numAtoms: u64,
    numShellsPerAtom: *const u64,
    shells: *const cuestAOShell_t,
    parameters: cuestAOBasisParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outBasis: *mut cuestAOBasis_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAOBasisCreate
        .expect("cuEST function `cuestAOBasisCreate` not found in the loaded library")(
        handle,
        numAtoms,
        numShellsPerAtom,
        shells,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outBasis,
    )
}

#[doc = " @brief Query workspace requirements for creating an AO basis object.\n @ingroup Basis\n\n This function simulates the allocation logic to determine necessary persistent and temporary buffer sizes.\n All buffer pointers and arrays must be valid.\n\n @param[in]  handle                          cuEST handle. Must not be NULL.\n @param[in]  numAtoms                        Total number of atoms/centers; must be greater than zero.\n @param[in]  numShellsPerAtom                Array specifying number of shells per center (size: \\c numAtoms); each must be greater than zero. CPU array, must not be NULL.\n @param[in]  shells                          Flat array of shell handles (\\c cuestAOShell_t), length is sum of \\c numShellsPerAtom.\n                                             CPU array, the array must not be NULL and none of the entries may be NULL.\n @param[in]  parameters                      Handle to \\c cuestAOBasisParameters_t. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor   Output descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor    Output descriptor for temporary buffer sizing. Must not be NULL.\n @param      outBasis                        Pointer for output basis handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle, any shell handle or parameters are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms == 0 or any \\c numShellsPerAtom[i]==0;\n   - \\c CUEST_STATUS_UNSUPPORTED_ARGUMENT if the AO basis mixes pure (solid-harmonic) and Cartesian shells;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for other errors."]
pub unsafe fn cuestAOBasisCreateWorkspaceQuery(
    handle: cuestHandle_t,
    numAtoms: u64,
    numShellsPerAtom: *const u64,
    shells: *const cuestAOShell_t,
    parameters: cuestAOBasisParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outBasis: *mut cuestAOBasis_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAOBasisCreateWorkspaceQuery
        .expect("cuEST function `cuestAOBasisCreateWorkspaceQuery` not found in the loaded library")(
        handle,
        numAtoms,
        numShellsPerAtom,
        shells,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outBasis,
    )
}

#[doc = " @brief Destroy/free an AO basis set object and all associated resources.\n @ingroup Basis\n\n After this function is called, the basis handle must not be used.\n\n @param[in] basis    AO basis set handle to destroy; must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if basis is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST AO basis handle is not of type \\c cuestAOBasis_t;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAOBasisDestroy(basis: cuestAOBasis_t) -> cuestStatus_t {
    dyload_lib()
        .cuestAOBasisDestroy
        .expect("cuEST function `cuestAOBasisDestroy` not found in the loaded library")(basis)
}

#[doc = " @brief Build an AO pair list structure.\n @ingroup Basis\n\n Constructs a sparse or dense pair list for efficient evaluation of integrals over AOs.\n All input pointers and buffers must be valid and not NULL.\n The number of atoms and xyz array length must match.\n Workspace buffers must be preallocated as returned by \\c cuestAOPairListCreateWorkspaceQuery.\n\n The cuestAOBasis_t basis handle used to construct the cuestAOPairList_t pair list handle\n is not needed for the pair list handle to be valid. However, in practice, use of the pair list\n will also require the cuestAOBasis_t basis handle and it should not be destroyed after creating\n the pair list.\n\n @param[in]  handle              cuEST handle. Must not be NULL.\n @param[in]  basis               AO basis set handle (object describing basis for the atoms). Must not be NULL.\n @param[in]  numAtoms            Number of atoms in molecule or fragment. Must be greater than zero.\n @param[in]  xyz                 Array of atomic Cartesian coordinates (length \\c numAtoms * 3) on the CPU. Ordering of elements is row-major: \\f$ [x_0, y_0, z_0, ...] \\f$ . Must not be NULL.\n @param[in]  thresholdPQ         Sparsity threshold for pair sieve (e.g., 1e-10). Must be non-negative.\n @param[in]  parameters          AO pair list parameters structure. Must not be NULL.\n @param[in]  persistentWorkspace Pointer to persistent workspace (preallocated). Must not be NULL.\n @param[in]  temporaryWorkspace  Pointer to temporary workspace (preallocated). Must not be NULL.\n @param[out] outPairList         Output pointer for AO pair list handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms == 0 or \\c thresholdPQ is negative;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAOPairListCreate(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    numAtoms: u64,
    xyz: *const f64,
    thresholdPQ: f64,
    parameters: cuestAOPairListParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outPairList: *mut cuestAOPairList_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAOPairListCreate
        .expect("cuEST function `cuestAOPairListCreate` not found in the loaded library")(
        handle,
        basis,
        numAtoms,
        xyz,
        thresholdPQ,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outPairList,
    )
}

#[doc = " @brief Query workspace requirements for AO pair list construction.\n @ingroup Basis\n\n Simulates memory usage to determine sizes of persistent and temporary buffers.\n All buffer and parameter pointers must be valid.\n\n @param[in]  handle                             cuEST handle. Must not be NULL.\n @param[in]  basis                              AO basis set handle (object describing basis for the atoms); must not be NULL.\n @param[in]  numAtoms                           Number of atoms. Must be greater than zero.\n @param[in]  xyz                                Array of atomic Cartesian coordinates (length \\c numAtoms * 3) on the CPU. Ordering of elements is \\f$ [x_0, y_0, z_0, ...] \\f$. Must not be NULL.\n @param[in]  thresholdPQ                        Sparsity threshold for pair sieve (e.g., 1e-10). Must be non-negative.\n @param[in]  parameters                         AO pair list parameters structure. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor      Output workspace descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor       Output workspace descriptor for temporary buffer sizing. Must not be NULL.\n @param      outPairList                        Output pointer for AO pair list handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms == 0 or \\c thresholdPQ is negative;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestAOPairListCreateWorkspaceQuery(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    numAtoms: u64,
    xyz: *const f64,
    thresholdPQ: f64,
    parameters: cuestAOPairListParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outPairList: *mut cuestAOPairList_t,
) -> cuestStatus_t {
    dyload_lib().cuestAOPairListCreateWorkspaceQuery.expect(
        "cuEST function `cuestAOPairListCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        basis,
        numAtoms,
        xyz,
        thresholdPQ,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outPairList,
    )
}

#[doc = " @brief Destroy/free an AO pair list object and its resources.\n @ingroup Basis\n\n The AO pair list handle and associated resources must not be used after this call.\n\n @param[in] pairList AO pair list handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if pairList is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST AOPairList handle is not of type \\c cuestAOPairList_t;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAOPairListDestroy(pairList: cuestAOPairList_t) -> cuestStatus_t {
    dyload_lib()
        .cuestAOPairListDestroy
        .expect("cuEST function `cuestAOPairListDestroy` not found in the loaded library")(
        pairList
    )
}

#[doc = " @brief Compute the effective core potential integral matrix using the provided plan.\n @ingroup ECPCompute\n\n The output buffer must be preallocated and sized for (nao × nao) elements, where nao is the\n number of atomic orbitals in the basis. Temporary workspace must be allocated using size\n returned by \\c cuestECPComputeWorkspaceQuery.\n\n @param[in]      handle                      cuEST handle. Must not be NULL.\n @param[in]      plan                        ECP integral computation plan (opaque handle). Must not be NULL.\n @param[in]      parameters                  Handle to \\c cuestECPComputeParameters_t. Must not be NULL.\n @param[in]      variableBufferSize          User-provided descriptor for the amount of memory used to evaluate the ECP integrals.\n                                                 2 GB is a good default buffer size for device memory. If an insufficient amount\n                                                 of memory is provided, the cuestECPCompute call will return CUEST_STATUS_EXCEPTION.\n                                                 If this occurs, increase the variableBufferSize and make the WorkspaceQuery and Compute calls\n                                                 again. This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n                                                 Host memory is currently unused. Must not be NULL.\n @param[in]      temporaryWorkspace          Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[out]     outECPMatrix                Output buffer for computed ECP matrix elements (size: nao × nao) on the GPU. Must not be NULL.\n                                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise;\n   - \\c CUEST_STATUS_EXCEPTION often indicates variableBufferSize is too small."]
pub unsafe fn cuestECPCompute(
    handle: cuestHandle_t,
    plan: cuestECPIntPlan_t,
    parameters: cuestECPComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outECPMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPCompute
        .expect("cuEST function `cuestECPCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        outECPMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for effective core potential matrix calculation.\n @ingroup ECPCompute\n\n This function computes the workspace requirements for a requested ECP calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outECPMatrix pointer may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              ECP integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Handle to \\c cuestECPComputeParameters_t. Must not be NULL.\n @param[in]  variableBufferSize                User-provided descriptor for the amount of memory used to evaluate the ECP integrals.\n                                               2 GB is a good default buffer size for device memory. If an insufficient amount\n                                               of memory is provided, the cuestECPCompute call will return CUEST_STATUS_EXCEPTION.\n                                               If this occurs, increase the variableBufferSize and make the WorkspaceQuery and Compute calls\n                                               again. This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n                                               Host memory is currently unused. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      outECPMatrix                      User matrix buffer. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestECPComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestECPIntPlan_t,
    parameters: cuestECPComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outECPMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPComputeWorkspaceQuery
        .expect("cuEST function `cuestECPComputeWorkspaceQuery` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        outECPMatrix,
    )
}

#[doc = " @brief Compute the electronic gradient (derivative) of the effective core potential integral matrix\n using the provided plan and density matrix.\n @ingroup ECPCompute\n\n Both the input density matrix and the output gradient must be preallocated. The density\n matrix must be an (nao x nao) array and the output gradient must have space for (natom x 3)\n elements, where nao is the number of atomic orbitals and natom the number of atoms in the system.\n All required temporary workspace must be allocated by the user prior to calling this function\n using size returned by \\c cuestECPDerivativeComputeWorkspaceQuery.\n\n @param[in]      handle                       cuEST handle. Must not be NULL.\n @param[in]      plan                         ECP integral computation plan (opaque handle). Must not be NULL.\n @param[in]      parameters                   Handle to \\c cuestECPDerivativeComputeParameters_t. Must not be NULL.\n @param[in]      variableBufferSize           User-provided descriptor for the amount of memory used to evaluate the ECP integrals.\n                                                  2 GB is a good default buffer size for device memory. If an insufficient amount\n                                                  of memory is provided, the cuestECPDerivativeCompute call will return CUEST_STATUS_EXCEPTION.\n                                                  If this occurs, increase the variableBufferSize and make the WorkspaceQuery and Compute calls\n                                                  again. This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n                                                  Host memory is currently unused. Must not be NULL.\n @param[in]      temporaryWorkspace           Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]      densityMatrix                Pointer to input density matrix (size: nao × nao) on the GPU. Must not be NULL.\n @param[out]     outGradient                  Output buffer for computed ECP gradient (size: natom × 3) on the GPU. Must not be NULL.\n                                                  Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise;\n   - \\c CUEST_STATUS_EXCEPTION often indicates variableBufferSize is too small."]
pub unsafe fn cuestECPDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestECPIntPlan_t,
    parameters: cuestECPDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPDerivativeCompute
        .expect("cuEST function `cuestECPDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for electronic gradient (derivative) of the\n effective core potential matrix using the supplied integral plan and density matrix.\n @ingroup ECPCompute\n\n This function computes the workspace requirements for a requested ECP gradient calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outGradient pointer may be NULL.\n\n @param[in]      handle                        cuEST handle. Must not be NULL.\n @param[in]      plan                          ECP integral computation plan (opaque handle). Must not be NULL.\n @param[in]      parameters                    Handle to \\c cuestECPDerivativeComputeParameters_t. Must not be NULL.\n @param[in]      variableBufferSize            User-provided descriptor for the amount of memory used to evaluate the ECP integrals.\n                                                   2 GB is a good default buffer size for device memory. If an insufficient amount\n                                                   of memory is provided, the cuestECPDerivativeCompute call will return CUEST_STATUS_EXCEPTION.\n                                                   If this occurs, increase the variableBufferSize and make the WorkspaceQuery and Compute calls\n                                                   again. This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n                                                   Host memory is currently unused. Must not be NULL.\n @param[out]     temporaryWorkspaceDescriptor  Output descriptor for temporary workspace sizes. Must not be NULL.\n @param          densityMatrix                 Pointer to input density matrix. May be NULL.\n @param          outGradient                   User buffer. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestECPDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestECPIntPlan_t,
    parameters: cuestECPDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestECPDerivativeComputeWorkspaceQuery.expect(
        "cuEST function `cuestECPDerivativeComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Create an effective core potential atom from a set of ECPShell.\n @ingroup ECP\n\n An ECP Atom comprises multiple ECP shells, grouped by atomic center.\n - All buffer pointers must be valid (not NULL).\n - Each shell, and each array entry, must be valid.\n - The output handle is only valid after successful creation.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  numElectrons          Total number of electrons corresponding to the ECP for this atom; must be greater than zero and even.\n @param[in]  numShells             Specifies number of shells for this atom with \\f$ L < L_{max} \\f$ ; must be greater than zero. \\f$ L_{max} \\f$ is equal to \\c numShells.\n @param[in]  shells                Flat CPU array of shell handles \\c cuestECPShell_t (size: \\c numShells). Must not be NULL and none of the entries may be NULL. The array must contain exactly one shell per angular momentum in ascending order, i.e. the entry at index \\c i must have \\f$ L == i \\f$.\n @param[in]  topShell              Shell handle (\\c cuestECPShell_t) with \\f$ L == L_{max} == \\f$ \\c numShells for this atom. Must not be NULL.\n @param[in]  parameters            Handle to \\c cuestECPAtomParameters_t. Must not be NULL.\n @param[out] outECPAtom            Pointer for output atom handle. Must not be NULL.\n\n All objects on CPU.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer parameter is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numElectrons == 0 or \\c numShells == 0;\n   - \\c CUEST_STATUS_EXCEPTION if \\c numElectrons is odd, a shell is out of order (\\c shells[i] with \\f$ L != i \\f$), or \\c topShell does not have \\f$ L == \\f$ \\c numShells;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on other errors."]
pub unsafe fn cuestECPAtomCreate(
    handle: cuestHandle_t,
    numElectrons: u64,
    numShells: u64,
    shells: *const cuestECPShell_t,
    topShell: cuestECPShell_t,
    parameters: cuestECPAtomParameters_t,
    outECPAtom: *mut cuestECPAtom_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPAtomCreate
        .expect("cuEST function `cuestECPAtomCreate` not found in the loaded library")(
        handle,
        numElectrons,
        numShells,
        shells,
        topShell,
        parameters,
        outECPAtom,
    )
}

#[doc = " @brief Destroy/free an ECP Atom object and all associated resources.\n @ingroup ECP\n\n After this function is called, the ECP atom handle must not be used.\n\n @param[in] atom    ECP Atom handle to destroy; must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if atom is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST ECP Atom handle is not of type \\c cuestECPAtom_t;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestECPAtomDestroy(atom: cuestECPAtom_t) -> cuestStatus_t {
    dyload_lib()
        .cuestECPAtomDestroy
        .expect("cuEST function `cuestECPAtomDestroy` not found in the loaded library")(atom)
}

#[doc = " @brief Create an ECP shell object with specified parameters.\n @ingroup ECP\n\n @param[in]      handle             cuEST handle. Must not be NULL.\n @param[in]      L                  Angular momentum quantum number (s=0, p=1, d=2, etc).\n @param[in]      numPrimitive       Number of primitives; must be greater than zero.\n @param[in]      radialPowers       Pointer to array containing radial powers (size: \\c numPrimitive) on the CPU. Elements must be non-negative integers, e.g. >= 0. Must not be NULL.\n @param[in]      coefficients       Pointer to array containing coefficients (size: \\c numPrimitive) on the CPU. Must not be NULL.\n @param[in]      exponents          Pointer to array containing exponents (size: \\c numPrimitive) on the CPU. Elements must be strictly positive and finite, e.g. > 0. Must not be NULL.\n @param[in]      parameters         Handle to \\c cuestECPShellParameters_t. Must not be NULL.\n @param[out]     outECPShell        Pointer to output shell object; receives opaque handle on success. Must not be NULL.\n\n All objects on CPU.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle or parameters are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numPrimitive is zero, or if any exponent is not strictly positive and finite (> 0);\n   - \\c CUEST_STATUS_EXCEPTION for unexpected error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR for non-std exceptions."]
pub unsafe fn cuestECPShellCreate(
    handle: cuestHandle_t,
    L: u64,
    numPrimitive: u64,
    radialPowers: *const u64,
    coefficients: *const f64,
    exponents: *const f64,
    parameters: cuestECPShellParameters_t,
    outECPShell: *mut cuestECPShell_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPShellCreate
        .expect("cuEST function `cuestECPShellCreate` not found in the loaded library")(
        handle,
        L,
        numPrimitive,
        radialPowers,
        coefficients,
        exponents,
        parameters,
        outECPShell,
    )
}

#[doc = " @brief Destroy/free an ECP shell object created with \\c cuestECPShellCreate.\n @ingroup ECP\n\n Frees all memory associated with the ECP shell. After destruction, the shell handle must not be used.\n\n @param[in] shell   Opaque ECP shell handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c shell is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the \\c shell handle is not of the \\c cuestECPShell_t type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestECPShellDestroy(shell: cuestECPShell_t) -> cuestStatus_t {
    dyload_lib()
        .cuestECPShellDestroy
        .expect("cuEST function `cuestECPShellDestroy` not found in the loaded library")(shell)
}

#[doc = " @brief Build a plan for effective core potential integrals.\n @ingroup ECPIntegralPlan\n\n All object handles, workspaces, and output pointers must be valid (not NULL).\n Workspace buffers must be preallocated with required sizes (see workspace query).\n\n \\verbatim embed:rst\n.. Note::\n- The number of atoms in the `cuestAOBasis_t` basis handle must match the length of the `xyzCPU` array.\n- The `cuestAOBasis_t` basis handle should define a basis set that is constructed to use the ECP functions\nbeing defined here. For example if the basis set includes a 1s orbital on a particular atom and that\n1s orbital is modelled by the ECPs included here, the downstream results may be undesirable. No checking\nis done for such a situation.\n- It is the user's responsibility to modify other terms that may relate to the presence of ECPs, such as\nthe potential integrals. This will not be done automatically.\n\\endverbatim\n\n @param[in]   handle               cuEST handle. Must not be NULL.\n @param[in]   basis                AO basis set handle. Must not be NULL.\n @param[in]   xyzCPU               Coordinates for full system (size: natom x 3) on the CPU. Must not be NULL.\n @param[in]   numECPAtoms          Number of atoms with ECPs. Must be greater than zero and less than or equal to total number of system atoms, natom.\n @param[in]   activeIndices        Indices of atoms with ECPs relative to full list of atoms (size: numECPAtoms) on the CPU. Must not be NULL.\n                                   The \\c activeIndices cannot contain indices greater than or equal to natom and cannot contain duplicate entries.\n @param[in]   activeAtoms          CPU array of ECP atom handles. Must not be NULL and none of the entries may be NULL.\n @param[in]   parameters           Plan parameter structure. Must not be NULL.\n @param[in]   persistentWorkspace  Preallocated workspace for persistent buffers. Must not be NULL.\n @param[in]   temporaryWorkspace   Preallocated workspace for temporary buffers. Must not be NULL.\n @param[out]  outPlan              Output pointer for integral plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numECPAtoms is zero or the \\c activeIndices are invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestECPIntPlanCreate(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    xyzCPU: *const f64,
    numECPAtoms: u64,
    activeIndices: *const u64,
    activeAtoms: *const cuestECPAtom_t,
    parameters: cuestECPIntPlanParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outPlan: *mut cuestECPIntPlan_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestECPIntPlanCreate
        .expect("cuEST function `cuestECPIntPlanCreate` not found in the loaded library")(
        handle,
        basis,
        xyzCPU,
        numECPAtoms,
        activeIndices,
        activeAtoms,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outPlan,
    )
}

#[doc = " @brief Query workspace requirements for creating an effective core potential integral plan.\n @ingroup ECPIntegralPlan\n\n Determines persistent and temporary buffer sizes by simulating allocation logic.\n All handles and pointers must be valid (not NULL).\n\n @param[in]  handle                             cuEST handle. Must not be NULL.\n @param[in]  basis                              AO basis set handle. Must not be NULL.\n @param[in]  xyzCPU                             Coordinates for full system (size: natom x 3) on the CPU. Must not be NULL.\n @param[in]  numECPAtoms                        Number of atoms with ECPs. Must be greater than zero and less than or equal to total number of system atoms, natom.\n @param[in]  activeIndices                      Indices of atoms with ECPs relative to full list of atoms (size: numECPAtoms) on the CPU. Must not be NULL.\n                                                The \\c activeIndices cannot contain indices greater than or equal to natom and cannot contain duplicate entries.\n @param[in]  activeAtoms                        CPU array of ECP atom handles. Must not be NULL and none of the entries may be NULL.\n @param[in]  parameters                         Plan parameters handle. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor      Output descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor       Output descriptor for temporary buffer sizing. Must not be NULL.\n @param      outPlan                            Pointer for integral plan handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numECPAtoms is zero or the \\c activeIndices are invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestECPIntPlanCreateWorkspaceQuery(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    xyzCPU: *const f64,
    numECPAtoms: u64,
    activeIndices: *const u64,
    activeAtoms: *const cuestECPAtom_t,
    parameters: cuestECPIntPlanParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outPlan: *mut cuestECPIntPlan_t,
) -> cuestStatus_t {
    dyload_lib().cuestECPIntPlanCreateWorkspaceQuery.expect(
        "cuEST function `cuestECPIntPlanCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        basis,
        xyzCPU,
        numECPAtoms,
        activeIndices,
        activeAtoms,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outPlan,
    )
}

#[doc = " @brief Destroy and release an effective core potential integral plan and its resources.\n @ingroup ECPIntegralPlan\n\n The plan handle and its memory must not be used after destruction.\n\n @param[in] plan   Plan handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c plan is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestECPIntPlanDestroy(plan: cuestECPIntPlan_t) -> cuestStatus_t {
    dyload_lib()
        .cuestECPIntPlanDestroy
        .expect("cuEST function `cuestECPIntPlanDestroy` not found in the loaded library")(plan)
}

#[doc = " @brief Build a plan for one-electron integrals over atomic orbitals.\n @ingroup OneElectronIntegralPlan\n\n All object handles, workspaces, and output pointers must be valid (not NULL).\n Workspace buffers must be preallocated with required sizes (see workspace query).\n\n \\verbatim embed:rst\n.. Note::\n- The `cuestAOBasis_t` basis handle provided here must correspond to the basis handle provided\nwhen the `cuestAOPairList_t` pair list handle was created.\n- It is the user's responsibility not to destroy the `cuestAOBasis_t` basis handle or\n`cuestAOPairList_t` pair list handle until operations involving the `cuestOEIntPlan_t` plan\nhandle are complete.\n- After the `cuestAOBasis_t` basis handle or `cuestAOPairList_t` pair list handle are destroyed,\nthe `cuestOEIntPlan_t` plan handle is no longer valid.\n- Screening of negligible integrals computed while using this `cuestOEIntPlan_t` plan handle\nis controlled by `thresholdPQ` set during construction of the `cuestAOPairList_t` pair list handle.\n\\endverbatim\n\n @param[in]  handle               cuEST handle. Must not be NULL.\n @param[in]  basis                AO basis set handle. Must not be NULL.\n @param[in]  pairList             AO pair list handle. Must not be NULL.\n @param[in]  parameters           Plan parameter structure. Must not be NULL.\n @param[in]  persistentWorkspace  Preallocated workspace for persistent buffers. Must not be NULL.\n @param[in]  temporaryWorkspace   Preallocated workspace for temporary buffers. Must not be NULL.\n @param[out] outPlan              Output pointer for integral plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOEIntPlanCreate(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    pairList: cuestAOPairList_t,
    parameters: cuestOEIntPlanParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outPlan: *mut cuestOEIntPlan_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestOEIntPlanCreate
        .expect("cuEST function `cuestOEIntPlanCreate` not found in the loaded library")(
        handle,
        basis,
        pairList,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outPlan,
    )
}

#[doc = " @brief Query workspace requirements for creating a one-electron integral plan.\n @ingroup OneElectronIntegralPlan\n\n Determines persistent and temporary buffer sizes by simulating allocation logic.\n All handles and pointers must be valid (not NULL).\n\n @param[in]  handle                             cuEST handle. Must not be NULL.\n @param[in]  basis                              AO basis set handle. Must not be NULL.\n @param[in]  pairList                           AO pair list handle. Must not be NULL.\n @param[in]  parameters                         Plan parameters handle. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor      Output descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor       Output descriptor for temporary buffer sizing. Must not be NULL.\n @param      outPlan                            Output pointer for integral plan handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOEIntPlanCreateWorkspaceQuery(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    pairList: cuestAOPairList_t,
    parameters: cuestOEIntPlanParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outPlan: *mut cuestOEIntPlan_t,
) -> cuestStatus_t {
    dyload_lib().cuestOEIntPlanCreateWorkspaceQuery.expect(
        "cuEST function `cuestOEIntPlanCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        basis,
        pairList,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outPlan,
    )
}

#[doc = " @brief Destroy and release a one-electron integral plan and its resources.\n @ingroup OneElectronIntegralPlan\n\n The plan handle and its memory must not be used after destruction.\n\n @param[in] plan   Plan handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c plan is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOEIntPlanDestroy(plan: cuestOEIntPlan_t) -> cuestStatus_t {
    dyload_lib()
        .cuestOEIntPlanDestroy
        .expect("cuEST function `cuestOEIntPlanDestroy` not found in the loaded library")(plan)
}

#[doc = " @brief Build a density-fitted integral plan (DF) for atomic orbitals.\n @ingroup DensityFittingIntegralPlan\n\n Constructs all needed metric arrays and three-center integrals. All handles and buffers must be valid.\n Workspace buffers must be preallocated using sizes from workspace query.\n\n The \\c cuestDFIntPlan_t may be configured to treat range-separated Coulomb operators.\n\n \\f[\n   \\underbrace{\\frac{1}{r}}_{\\text{full-range}}\n   =\n   \\underbrace{\\frac{\\mathrm{erfc}(\\omega r)}{r}}_{\\text{short-range}}\n   +\n   \\underbrace{\\frac{\\mathrm{erf}(\\omega r)}{r}}_{\\text{long-range}}\n \\f]\n\n In cuEST, range-separation is parameterized in terms of the full-range and long-range components.\n\n - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION: Controls the fraction of full-range exchange included.\n - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION: Controls the fraction of long-range exchange included.\n - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA: Specifies the range separation parameter.\n\n It is required that the operator specification must be positive and respect the following conditions:\n\n - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION \\f$\\ge 0\\f$.\n - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION + \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION \\f$\\ge 0\\f$.\n - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA \\f$\\ge 0\\f$.\n\n \\c CUEST_STATUS_INVALID_ARGUMENT will be returned if these conditions are not respected during handle creation.\n\n The operator specification is respected by all subsequent calls involving the \\c cuestDFIntPlan_t. The one\n exception is \\c cuestDFCoulombCompute, which is always evaluated using the full-range Coulomb operator.\n\n Example configurations:\n\n - PBE0 (full-range, only):\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION \\f$= 0.25\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION \\f$= 0\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA unused.\n\n - \\f$\\omega\\f$B97 (long-range, only):\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION \\f$= 0.0\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION \\f$= 1.0\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA \\f$= 0.4\\f$.\n\n - \\f$\\omega\\f$B97M-V (full- and long-range):\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION \\f$= 0.15\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION \\f$= 0.85\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA \\f$= 0.3\\f$.\n\n - HSE06 (short-range, only):\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION \\f$= 0.25\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION \\f$= -0.25\\f$.\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA \\f$= 0.11\\f$.\n\n It is possible to put or create the \\c cuestDFIntPlan_t in a state where it is no longer possible to use\n the \\c cuestDFIntPlan_t for certain calculations.\n\n - Creating the \\c cuestDFIntPlan_t with the \\c CUEST_DFINTPLAN_PARAMETERS_THREE_INDEX_INTEGRAL_DIRECT parameter\n   set to a non-zero value allows the plan to be used in calls to \\c cuestDFMOIntegralsCompute. At present, no\n   other functions support this configuration.\n - Calls to \\c cuestDFSymmetricDerivativeCompute using the \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_MEMORY_POLICY_OVERWRITE\n   memory policy place the \\c cuestDFIntPlan_t in a dirty state. No further compute calls involving this plan will\n   succeed.\n\n \\c CUEST_STATUS_INVALID_PLAN will be returned if a compute call is made with a \\c cuestDFIntPlan_t in an\n unusable state.\n\n \\verbatim embed:rst\n.. Note::\n- The number of atoms in the primary basis handle must match the number of atoms in the\nauxiliary basis handle.\n- The `cuestAOBasis_t` primary basis handle provided here must correspond to the basis handle provided\nwhen the `cuestAOPairList_t` pair list handle was created.\n- It is the user's responsibility not to destroy the `cuestAOBasis_t` primary basis handle,\nauxiliary basis handle or `cuestAOPairList_t` pair list handle until operations involving the\n`cuestDFIntPlan_t` plan handle are complete.\n- After the `cuestAOBasis_t` basis handle, auxiliary basis handle or `cuestAOPairList_t` pair list handle are destroyed, the `cuestDFIntPlan_t` plan handle is no longer valid.\n- Screening of negligible integrals computed while using this `cuestDFIntPlan_t` plan handle\nis controlled by `thresholdPQ` set during construction of the `cuestAOPairList_t` pair list handle.\n\\endverbatim\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  primaryBasis          AO basis set for primary orbitals. Must not be NULL.\n @param[in]  auxiliaryBasis        AO basis set for auxiliary/fitting orbitals.\n                                       The auxiliary basis set must use a pure (spherical)\n                                       angular momentum representation. Must not be NULL.\n @param[in]  pairList              AO pair list describing sparsity. Must not be NULL.\n @param[in]  parameters            Density-fitting parameters structure. Must not be NULL.\n @param[in]  persistentWorkspace   Workspace for persistent buffers. Must not be NULL.\n @param[in]  temporaryWorkspace    Workspace for temporary buffers. Must not be NULL.\n @param[out] outPlan               Output pointer for DF plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if any of the \\c cuestDFIntPlanParameters_t are invalid;\n   - \\c CUEST_STATUS_EXCEPTION if \\c auxiliaryBasis is not pure;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFIntPlanCreate(
    handle: cuestHandle_t,
    primaryBasis: cuestAOBasis_t,
    auxiliaryBasis: cuestAOBasis_t,
    pairList: cuestAOPairList_t,
    parameters: cuestDFIntPlanParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outPlan: *mut cuestDFIntPlan_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestDFIntPlanCreate
        .expect("cuEST function `cuestDFIntPlanCreate` not found in the loaded library")(
        handle,
        primaryBasis,
        auxiliaryBasis,
        pairList,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outPlan,
    )
}

#[doc = " @brief Query workspace sizes for creating a density-fitted integral plan.\n @ingroup DensityFittingIntegralPlan\n\n Simulates all allocations for DF plan, returning size requirements.\n All handles and pointers must be valid (not NULL). Output descriptors are filled on success.\n\n @param[in]  handle                             cuEST handle. Must not be NULL.\n @param[in]  primaryBasis                       AO basis set for primary orbitals. Must not be NULL.\n @param[in]  auxiliaryBasis                     AO basis set for auxiliary/fitting orbitals.\n                                                    The auxiliary basis set must use a pure (spherical)\n                                                    angular momentum representation. Must not be NULL.\n @param[in]  pairList                           AO pair list handle. Must not be NULL.\n @param[in]  parameters                         Density-fitting parameters structure. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor      Output descriptor for persistent buffers. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor       Output descriptor for temporary buffers. Must not be NULL.\n @param      outPlan                            Output pointer for DF plan handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if any of the \\c cuestDFIntPlanParameters_t are invalid;\n   - \\c CUEST_STATUS_EXCEPTION if \\c auxiliaryBasis is not pure;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFIntPlanCreateWorkspaceQuery(
    handle: cuestHandle_t,
    primaryBasis: cuestAOBasis_t,
    auxiliaryBasis: cuestAOBasis_t,
    pairList: cuestAOPairList_t,
    parameters: cuestDFIntPlanParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outPlan: *mut cuestDFIntPlan_t,
) -> cuestStatus_t {
    dyload_lib().cuestDFIntPlanCreateWorkspaceQuery.expect(
        "cuEST function `cuestDFIntPlanCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        primaryBasis,
        auxiliaryBasis,
        pairList,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outPlan,
    )
}

#[doc = " @brief Destroy/free a density-fitted integral plan and all resources.\n @ingroup DensityFittingIntegralPlan\n\n The plan handle and its memory must not be used after destruction.\n\n @param[in] plan      DF plan handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c plan is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFIntPlanDestroy(plan: cuestDFIntPlan_t) -> cuestStatus_t {
    dyload_lib()
        .cuestDFIntPlanDestroy
        .expect("cuEST function `cuestDFIntPlanDestroy` not found in the loaded library")(plan)
}

#[doc = " @brief Compute the atomic orbital overlap matrix (S) using the provided overlap plan.\n @ingroup Overlap\n\n The output buffer must be preallocated and sized for nao × nao elements, where nao is the\n number of atomic orbitals in the basis. Temporary workspace must be allocated using size\n returned by \\c cuestOverlapComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestOverlapComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[out] outSMatrix         Output GPU buffer for computed overlap matrix elements [size: nao × nao]. Must not be NULL.\n                                Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOverlapCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestOverlapComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outSMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestOverlapCompute
        .expect("cuEST function `cuestOverlapCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        outSMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for overlap matrix calculation.\n @ingroup Overlap\n\n This function computes the workspace requirements for a requested overlap calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outSMatrix pointer may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestOverlapComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      outSMatrix                        User matrix buffer (may be NULL during the workspace query).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOverlapComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestOverlapComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outSMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestOverlapComputeWorkspaceQuery.expect(
        "cuEST function `cuestOverlapComputeWorkspaceQuery` not found in the loaded library",
    )(handle, plan, parameters, temporaryWorkspaceDescriptor, outSMatrix)
}

#[doc = " @brief Compute the electronic gradient (derivative) of the atomic orbital overlap matrix\n        using the supplied integral plan and density matrix.\n @ingroup OverlapGrad\n\n Both the input density matrix and the output gradient must be preallocated. The density\n matrix must be an (nao x nao) array and the output gradient must have space for (natom x 3)\n elements, where nao is the number of atomic orbitals and natom the number of atoms in the system.\n All required temporary workspace must be allocated by the user prior to calling this function.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  plan                  One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters            Compute parameters (\\c cuestOverlapDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace    Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  densityMatrix         Pointer to input density matrix [size: nao × nao]. GPU array. Must not be NULL.\n @param[out] outGradient           Pointer to GPU buffer for computed gradients [size: natom × 3]. Must not be NULL.\n                                   Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOverlapDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestOverlapDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestOverlapDerivativeCompute
        .expect("cuEST function `cuestOverlapDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for overlap matrix gradient calculation.\n @ingroup OverlapGrad\n\n This function computes the workspace requirements of a single gradient evaluation,\n based on the supplied plan and dimensions. The workspace descriptor is filled with host and device memory\n requirements. Output buffers may be NULL for the purposes of the query.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestOverlapDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      densityMatrix                     Optional density matrix pointer (may be NULL).\n @param      outGradient                       Optional buffer for computed gradient (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestOverlapDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestOverlapDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestOverlapDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestOverlapDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, densityMatrix, outGradient)
}

#[doc = " @brief Compute the atomic orbital kinetic energy matrix (T-matrix) using the supplied integral plan.\n @ingroup Kinetic\n\n The output buffer must be preallocated by the user and have space for (nao × nao) doubles,\n where nao is the number of atomic orbitals in the basis. All required temporary workspace\n must be allocated before calling this function, using the size returned from\n \\c cuestKineticComputeWorkspaceQuery. All pointers must be valid and not NULL.\n\n @param[in]  handle              cuEST handle. Must not be NULL.\n @param[in]  plan                One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters          Compute parameters (\\c cuestKineticComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace  Temporary workspace buffers for computation (preallocated).\n @param[out] outTMatrix          Output GPU buffer for computed kinetic matrix elements (size: nao × nao). Must not be NULL.\n                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestKineticCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestKineticComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outTMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestKineticCompute
        .expect("cuEST function `cuestKineticCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        outTMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for kinetic energy matrix calculation.\n @ingroup Kinetic\n\n This function determines the workspace requirements for a kinetic energy matrix evaluation\n (such as buffer sizes), given a particular plan. The output descriptor is filled on success.\n User matrix buffer may be NULL.\n\n @param[in]  handle                             cuEST handle. Must not be NULL.\n @param[in]  plan                               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                         Compute parameters (\\c cuestKineticComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor       Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      outTMatrix                         Optional buffer for computed kinetic matrix (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestKineticComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestKineticComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outTMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestKineticComputeWorkspaceQuery.expect(
        "cuEST function `cuestKineticComputeWorkspaceQuery` not found in the loaded library",
    )(handle, plan, parameters, temporaryWorkspaceDescriptor, outTMatrix)
}

#[doc = " @brief Compute the gradient (nuclear derivative) of the AO kinetic energy matrix (T)\n        with respect to atomic positions, contracted with a density matrix.\n @ingroup KineticGrad\n\n The user must provide a preallocated density matrix (size nao × nao) and a preallocated\n output gradient buffer (size natom × 3). All required temporary workspace must be sized and\n allocated using the query function below.\n\n @param[in]  handle              cuEST handle. Must not be NULL.\n @param[in]  plan                One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters          Compute parameters (\\c cuestKineticDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace  Temporary workspace buffers (preallocated for this operation). Must not be NULL.\n @param[in]  densityMatrix       Input density matrix (size: nao × nao). GPU array. Must not be NULL.\n @param[out] outGradient         Output GPU buffer for gradient (size: natom × 3). Must not be NULL.\n                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestKineticDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestKineticDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestKineticDerivativeCompute
        .expect("cuEST function `cuestKineticDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a kinetic energy gradient computation.\n @ingroup KineticGrad\n\n This function determines the workspace required for a\n kinetic energy gradient evaluation with a given plan. The output\n descriptor is filled on success.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestKineticDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes (host and device). Must not be NULL.\n @param      densityMatrix                     Optional density matrix (may be NULL).\n @param      outGradient                       Optional output gradient (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestKineticDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestKineticDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestKineticDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestKineticDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, densityMatrix, outGradient)
}

#[doc = " @brief Compute the atomic orbital multipole matrix using the provided \\c cuestOEIntPlan_t plan.\n @ingroup Multipole\n @since v0.2.0\n\n The output buffer must be preallocated and sized for nao × nao elements, where nao is the\n number of atomic orbitals in the basis. Temporary workspace must be allocated using size\n returned by \\c cuestMultipoleComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestMultipoleComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  multipoleOrder     Exponents defining the multipole operator: x^l y^m z^n [size: 3]. CPU array. Must not be NULL.\n @param[in]  origin             Cartesian coordinates defining the multipole origin [size: 3]. CPU array. Must not be NULL.\n @param[out] outMatrix          Output GPU buffer for computed multipole matrix elements [size: nao × nao]. Must not be NULL.\n                                Elements of this array are overwritten by the calculation results.\n\n With the default configuration of the cuEST handle, multipole moments with a total order of up to 30 are supported (assuming an\n orbital basis including g orbitals). Support for higher-order multipoles can be enabled by increasing \\c CUEST_HANDLE_PARAMETERS_MAX_GAUSS_HERMITE.\n Exceeding this limit will result in a \\c CUEST_STATUS_EXCEPTION return code.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestMultipoleCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestMultipoleComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    multipoleOrder: *const u32,
    origin: *const f64,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestMultipoleCompute
        .expect("cuEST function `cuestMultipoleCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        multipoleOrder,
        origin,
        outMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for multipole matrix calculation.\n @ingroup Multipole\n @since v0.2.0\n\n This function computes the workspace requirements for a requested multipole calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outMatrix pointer may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestMultipoleComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  multipoleOrder                    Integer exponents defining the multipole operator: x^l y^m z^n [size: 3]. CPU array. Must not be NULL.\n @param[in]  origin                            Cartesian coordinates defining the multipole origin [size: 3]. CPU array. Must not be NULL.\n @param      outMatrix                         User matrix buffer (may be NULL during the workspace query).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestMultipoleComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestMultipoleComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    multipoleOrder: *const u32,
    origin: *const f64,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestMultipoleComputeWorkspaceQuery.expect(
        "cuEST function `cuestMultipoleComputeWorkspaceQuery` not found in the loaded library",
    )(handle, plan, parameters, temporaryWorkspaceDescriptor, multipoleOrder, origin, outMatrix)
}

#[doc = " @brief Compute the electronic gradient (derivative) of the atomic orbital multipole matrix\n        using the supplied integral plan and density matrix.\n @ingroup MultipoleGrad\n @since v0.2.0\n\n Both the input density matrix and the output gradient must be preallocated. The density\n matrix must be an (nao x nao) array and the output gradient must have space for (natom x 3)\n elements, where nao is the number of atomic orbitals and natom the number of atoms in the system.\n All required temporary workspace must be allocated by the user prior to calling this function.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  plan                  One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters            Compute parameters (\\c cuestMultipoleDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace    Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  multipoleOrder        Exponents defining the multipole operator: x^l y^m z^n [size: 3]. CPU array. Must not be NULL.\n @param[in]  origin                Cartesian coordinates defining the multipole origin [size: 3]. CPU array. Must not be NULL.\n @param[in]  densityMatrix         Pointer to input density matrix [size: nao × nao]. GPU array. Must not be NULL.\n @param[out] outGradient           Pointer to GPU buffer for computed gradients [size: natom × 3]. Must not be NULL.\n                                   Elements of this array are overwritten by the calculation results.\n\n With the default configuration of the cuEST handle, multipole moment derivatives with a total order of up to 29 are supported (assuming an\n orbital basis including g orbitals). Support for higher-order multipoles can be enabled by increasing \\c CUEST_HANDLE_PARAMETERS_MAX_GAUSS_HERMITE.\n Exceeding this limit will result in a \\c CUEST_STATUS_EXCEPTION return code.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestMultipoleDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestMultipoleDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    multipoleOrder: *const u32,
    origin: *const f64,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestMultipoleDerivativeCompute
        .expect("cuEST function `cuestMultipoleDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        multipoleOrder,
        origin,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for multipole matrix gradient calculation.\n @ingroup MultipoleGrad\n @since v0.2.0\n\n This function computes the workspace requirements of a single gradient evaluation,\n based on the supplied plan and dimensions. The workspace descriptor is filled with host and device memory\n requirements. Output buffers may be NULL for the purposes of the query.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestMultipoleDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  multipoleOrder                    Exponents defining the multipole operator: x^l y^m z^n [size: 3]. CPU array. Must not be NULL.\n @param[in]  origin                            Cartesian coordinates defining the multipole origin [size: 3]. CPU array. Must not be NULL.\n @param      densityMatrix                     Optional density matrix pointer (may be NULL).\n @param      outGradient                       Optional buffer for computed gradient (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestMultipoleDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestMultipoleDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    multipoleOrder: *const u32,
    origin: *const f64,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestMultipoleDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestMultipoleDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, multipoleOrder, origin, densityMatrix, outGradient)
}

#[doc = " @brief Compute the atomic orbital angular momentum matrix using the provided \\c cuestOEIntPlan_t plan.\n @ingroup AngularMomentum\n @since v0.2.0\n\n The output buffer must be preallocated and sized for nao × nao elements, where nao is the\n number of atomic orbitals in the basis. Temporary workspace must be allocated using size\n returned by \\c cuestAngularMomentumComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestAngularMomentumComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  component          Enum (\\c cuestAngularMomentumComputeParametersComponent_t) defining the angular momentum operator (i.e. \\f$L_x\\f$, \\f$L_y\\f$, \\f$L_z\\f$).\n @param[in]  origin             Cartesian coordinates defining the operator origin [size: 3]. CPU array. Must not be NULL.\n @param[out] outMatrix          Output GPU buffer for computed angular momentum matrix elements [size: nao × nao]. Must not be NULL.\n                                Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestAngularMomentumComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestAngularMomentumCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestAngularMomentumComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    component: cuestAngularMomentumComputeParametersComponent_t,
    origin: *const f64,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAngularMomentumCompute
        .expect("cuEST function `cuestAngularMomentumCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        component,
        origin,
        outMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for angular momentum matrix calculation.\n @ingroup AngularMomentum\n @since v0.2.0\n\n This function computes the workspace requirements for a requested angular momentum calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outMatrix pointer may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestAngularMomentumComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  component                         Enum (\\c cuestAngularMomentumComputeParametersComponent_t) defining the angular momentum operator (i.e. \\f$L_x\\f$, \\f$L_y\\f$, \\f$L_z\\f$).\n @param[in]  origin                            Cartesian coordinates defining the operator origin [size: 3]. CPU array. Must not be NULL.\n @param      outMatrix                         User matrix buffer (may be NULL during the workspace query).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestAngularMomentumComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestAngularMomentumComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestAngularMomentumComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    component: cuestAngularMomentumComputeParametersComponent_t,
    origin: *const f64,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestAngularMomentumComputeWorkspaceQuery.expect("cuEST function `cuestAngularMomentumComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, component, origin, outMatrix)
}

#[doc = " @brief Compute the electronic gradient (derivative) of the atomic orbital angular momentum matrix\n        using the supplied integral plan and density matrix.\n @ingroup AngularMomentumGrad\n @since v0.2.0\n\n Both the input density matrix and the output gradient must be preallocated. The density\n matrix must be an (nao x nao) array and the output gradient must have space for (natom x 3)\n elements, where nao is the number of atomic orbitals and natom the number of atoms in the system.\n All required temporary workspace must be allocated by the user prior to calling this function.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  plan                  One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters            Compute parameters (\\c cuestAngularMomentumDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace    Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  component             Enum (\\c cuestAngularMomentumComputeParametersComponent_t) defining the angular momentum operator (i.e. \\f$L_x\\f$, \\f$L_y\\f$, \\f$L_z\\f$).\n @param[in]  origin                Cartesian coordinates defining the operator origin [size: 3]. CPU array. Must not be NULL.\n @param[in]  densityMatrix         Pointer to input density matrix [size: nao × nao]. GPU array. Must not be NULL.\n @param[out] outGradient           Pointer to GPU buffer for computed gradients [size: natom × 3]. Must not be NULL.\n                                   Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestAngularMomentumComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestAngularMomentumDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestAngularMomentumDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    component: cuestAngularMomentumComputeParametersComponent_t,
    origin: *const f64,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestAngularMomentumDerivativeCompute.expect(
        "cuEST function `cuestAngularMomentumDerivativeCompute` not found in the loaded library",
    )(
        handle, plan, parameters, temporaryWorkspace, component, origin, densityMatrix, outGradient
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for angular momentum matrix gradient calculation.\n @ingroup AngularMomentumGrad\n @since v0.2.0\n\n This function computes the workspace requirements of a single gradient evaluation,\n based on the supplied plan and dimensions. The workspace descriptor is filled with host and device memory\n requirements. Output buffers may be NULL for the purposes of the query.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestAngularMomentumDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  component                         Enum (\\c cuestAngularMomentumComputeParametersComponent_t) defining the angular momentum operator (i.e. \\f$L_x\\f$, \\f$L_y\\f$, \\f$L_z\\f$).\n @param[in]  origin                            Cartesian coordinates defining the operator origin [size: 3]. CPU array. Must not be NULL.\n @param      densityMatrix                     Optional density matrix pointer (may be NULL).\n @param      outGradient                       Optional buffer for computed gradient (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestAngularMomentumComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestAngularMomentumDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestAngularMomentumDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    component: cuestAngularMomentumComputeParametersComponent_t,
    origin: *const f64,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestAngularMomentumDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestAngularMomentumDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, component, origin, densityMatrix, outGradient)
}

#[doc = " @brief Compute the atomic orbital nabla matrix using the provided \\c cuestOEIntPlan_t plan.\n @ingroup Nabla\n @since v0.2.0\n\n The output buffer must be preallocated and sized for nao × nao elements, where nao is the\n number of atomic orbitals in the basis. Temporary workspace must be allocated using size\n returned by \\c cuestNablaComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestNablaComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  component          Enum (\\c cuestNablaComputeParametersComponent_t) defining the nabla operator (i.e. \\f$\\nabla_x\\f$, \\f$\\nabla_y\\f$, or \\f$\\nabla_z\\f$).\n @param[out] outMatrix          Output GPU buffer for computed nabla matrix elements [size: nao × nao]. Must not be NULL.\n                                Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestNablaComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNablaCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestNablaComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    component: cuestNablaComputeParametersComponent_t,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestNablaCompute
        .expect("cuEST function `cuestNablaCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        component,
        outMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for nabla matrix calculation.\n @ingroup Nabla\n @since v0.2.0\n\n This function computes the workspace requirements for a requested nabla calculation. Fill the\n provided workspace descriptor with host and device buffer sizes required. The \\c outMatrix pointer may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestNablaComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  component                         Enum (\\c cuestNablaComputeParametersComponent_t) defining the nabla operator (i.e. \\f$\\nabla_x\\f$, \\f$\\nabla_y\\f$, or \\f$\\nabla_z\\f$).\n @param      outMatrix                         User matrix buffer (may be NULL during the workspace query).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestNablaComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNablaComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestNablaComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    component: cuestNablaComputeParametersComponent_t,
    outMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestNablaComputeWorkspaceQuery
        .expect("cuEST function `cuestNablaComputeWorkspaceQuery` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspaceDescriptor,
        component,
        outMatrix,
    )
}

#[doc = " @brief Compute the electronic gradient (derivative) of the atomic orbital nabla matrix\n        using the supplied integral plan and density matrix.\n @ingroup NablaGrad\n @since v0.2.0\n\n Both the input density matrix and the output gradient must be preallocated. The density\n matrix must be an (nao x nao) array and the output gradient must have space for (natom x 3)\n elements, where nao is the number of atomic orbitals and natom the number of atoms in the system.\n All required temporary workspace must be allocated by the user prior to calling this function.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  plan                  One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters            Compute parameters (\\c cuestNablaDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace    Temporary workspace buffers for computation (preallocated). Must not be NULL.\n @param[in]  component             Enum (\\c cuestNablaComputeParametersComponent_t) defining the nabla operator (i.e. \\f$\\nabla_x\\f$, \\f$\\nabla_y\\f$, or \\f$\\nabla_z\\f$).\n @param[in]  densityMatrix         Pointer to input density matrix [size: nao × nao]. GPU array. Must not be NULL.\n @param[out] outGradient           Pointer to GPU buffer for computed gradients [size: natom × 3]. Must not be NULL.\n                                   Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestNablaComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNablaDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestNablaDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    component: cuestNablaComputeParametersComponent_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestNablaDerivativeCompute
        .expect("cuEST function `cuestNablaDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        component,
        densityMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for nabla matrix gradient calculation.\n @ingroup NablaGrad\n @since v0.2.0\n\n This function computes the workspace requirements of a single gradient evaluation,\n based on the supplied plan and dimensions. The workspace descriptor is filled with host and device memory\n requirements. Output buffers may be NULL for the purposes of the query.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestNablaDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  component                         Enum (\\c cuestNablaComputeParametersComponent_t) defining the nabla operator (i.e. \\f$\\nabla_x\\f$, \\f$\\nabla_y\\f$, or \\f$\\nabla_z\\f$).\n @param      densityMatrix                     Optional density matrix pointer (may be NULL).\n @param      outGradient                       Optional buffer for computed gradient (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c component is not a valid \\c cuestNablaComputeParametersComponent_t value;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNablaDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestNablaDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    component: cuestNablaComputeParametersComponent_t,
    densityMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNablaDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestNablaDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, component, densityMatrix, outGradient)
}

#[doc = " @brief Compute the atomic orbital nuclear attraction (potential energy) matrix (V) for given point charges.\n @ingroup Potential\n\n This function computes the AO potential energy matrix for a set of point charges (typically nuclei) as:\n    \\f[ V_{pq} = \\sum_{A} q_A * \\left< p \\left| \\frac{1}{|r - R_A|} \\right| q \\right> \\f]\n All input arrays must be preallocated and valid.\n   - The \\c xyz array has size \\c numCharges × 3 (row major order: \\f$ [x_0, y_0, z_0, x_1, y_1, z_1, ...] \\f$ ).\n   - The \\c q array has length \\c numCharges.\n   - The output buffer \\c outVMatrix must be preallocated for (nao × nao) doubles.\n Temporary workspace must be preallocated using the requirements from \\c cuestPotentialComputeWorkspaceQuery.\n\n Note:\n   - The potential is evaluated with a positive unit test charge. To obtain\n     nuclear attraction integrals, the \\c q array could have negative entries. Alternatively, the \\c q array\n     could have positive entries and the result scaled by the charge of an electron (i.e. -1 in atomic units).\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_POTENTIALCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_POTENTIALCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle              cuEST handle. Must not be NULL.\n @param[in]  plan                One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters          Compute parameters (\\c cuestPotentialComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace  Temporary workspace buffers, preallocated as required. Must not be NULL.\n @param[in]  numCharges          Number of point charges (must be greater than zero).\n @param[in]  xyz                 Array of charge coordinates (size: \\c numCharges × 3) in atomic units on GPU. Must not be NULL.\n @param[in]  q                   Array of charge magnitudes (size: \\c numCharges) on GPU. Must not be NULL.\n @param[out] outVMatrix          Output AO potential matrix buffer (size: nao × nao) on GPU. Must not be NULL.\n                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numCharges is zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPotentialCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestPotentialComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numCharges: u64,
    xyz: *const f64,
    q: *const f64,
    outVMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPotentialCompute
        .expect("cuEST function `cuestPotentialCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        numCharges,
        xyz,
        q,
        outVMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a potential energy matrix calculation.\n @ingroup Potential\n\n This function calculates the workspace required for a potential energy matrix evaluation,\n returning host/device workspace buffer sizes in the output descriptor. The user arrays \\c q, \\c xyz, and \\c outVMatrix may be NULL;\n only sizes are computed. \\c numCharges must be greater than zero.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_POTENTIALCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_POTENTIALCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle                              cuEST handle. Must not be NULL.\n @param[in]  plan                                One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                          Compute parameters (\\c cuestPotentialComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor        Output descriptor for temporary workspace sizes (host and device). Must not be NULL.\n @param[in]  numCharges                          Number of point charges (must be greater than zero.).\n @param      xyz                                 Optional array of charge coordinates (may be NULL).\n @param      q                                   Optional array of charge magnitudes (may be NULL).\n @param      outVMatrix                          Optional output AO potential matrix buffer (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numCharges is zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPotentialComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestPotentialComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numCharges: u64,
    xyz: *const f64,
    q: *const f64,
    outVMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPotentialComputeWorkspaceQuery.expect(
        "cuEST function `cuestPotentialComputeWorkspaceQuery` not found in the loaded library",
    )(handle, plan, parameters, temporaryWorkspaceDescriptor, numCharges, xyz, q, outVMatrix)
}

#[doc = " @brief Compute the potential integral derivatives with respect to atomic and charge coordinates,\n        contracted with a density matrix.\n @ingroup PotentialGrad\n\n This routine evaluates the derivative of the potential integrals and contracts the result with\n a density matrix in the AO basis. The resulting gradient is returned both with\n respect to the AO basis functions and with respect to the point-charge\n coordinates. All required temporary workspace must be sized and allocated\n using the corresponding workspace query function.\n\n The user must provide:\n - A preallocated/populated density matrix of size (nao × nao).\n - A list of point charges and their Cartesian coordinates.\n - Preallocated output buffers for the AO basis gradients and charge gradients (to be overwritten).\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_POTENTIALDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_POTENTIALDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - It is recommended to leave \\c CUEST_FFLOAT_USAGE_MODE_OFF for general use of the potential derivative.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestPotentialDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Temporary workspace buffers (preallocated for this operation). Must not be NULL.\n @param[in]  numCharges         Number of point charges. Must be greater than zero.\n @param[in]  xyz                Cartesian coordinates of the point charges\n                                    (size: \\c numCharges × 3, in row-major order) on GPU. Must not be NULL.\n @param[in]  q                  Point charges (size: \\c numCharges) on GPU. Must not be NULL.\n @param[in]  densityMatrix      Input density matrix (size: nao × nao) on GPU. Must not be NULL.\n @param[out] outBasisGradient   Output gradient with respect to AO basis centers\n                                    (size: natom × 3) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n @param[out] outChargeGradient  Output gradient with respect to the point-charge coordinates\n                                    (size: \\c numCharges × 3) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numCharges is zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPotentialDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestPotentialDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numCharges: u64,
    xyz: *const f64,
    q: *const f64,
    densityMatrix: *const f64,
    outBasisGradient: *mut f64,
    outChargeGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPotentialDerivativeCompute
        .expect("cuEST function `cuestPotentialDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        numCharges,
        xyz,
        q,
        densityMatrix,
        outBasisGradient,
        outChargeGradient,
    )
}

#[doc = " @brief Query the temporary workspace required to compute the potential integral\n        derivatives with respect to atomic and charge coordinates,\n        contracted with a density matrix.\n @ingroup PotentialGrad\n\n This function determines the workspace required for a potential derivative\n evaluation with a given plan and point-charge specification. The output\n descriptor is filled on success and can be used to allocate host and device\n workspace buffers prior to calling \\c cuestPotentialDerivativeCompute.\n\n The density matrix and output gradient pointers are optional and may be NULL.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_POTENTIALDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_POTENTIALDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - It is recommended to leave \\c CUEST_FFLOAT_USAGE_MODE_OFF for general use of the potential derivative.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              One-electron integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestPotentialDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes (host and device). Must not be NULL.\n @param[in]  numCharges                        Number of point charges. Must be greater than zero.\n @param      xyz                               Optional Cartesian coordinates of the point charges. May be NULL.\n @param      q                                 Optional point charges. May be NULL.\n @param      densityMatrix                     Optional density matrix. May be NULL.\n @param      outBasisGradient                  Optional output gradient buffer with respect to AO basis centers. May be NULL.\n @param      outChargeGradient                 Optional output gradient buffer with respect to the point-charge coordinates. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numCharges is zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPotentialDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestOEIntPlan_t,
    parameters: cuestPotentialDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numCharges: u64,
    xyz: *const f64,
    q: *const f64,
    densityMatrix: *const f64,
    outBasisGradient: *mut f64,
    outChargeGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPotentialDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestPotentialDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, numCharges, xyz, q, densityMatrix, outBasisGradient, outChargeGradient)
}

#[doc = " @brief Compute the AO Coulomb (J) matrix using density fitting (DF) with the supplied DF integral plan and density matrix.\n @ingroup Coulomb\n\n This routine computes the Coulomb matrix (J) from a density matrix using an auxiliary basis and\n density-fitted integrals. The user must provide a preallocated density matrix (size: nao × nao),\n a \\c cuestDFIntPlan_t, and a preallocated output buffer (size: nao × nao).\n All required temporary workspace must be allocated using the size returned by\n \\c cuestDFCoulombComputeWorkspaceQuery.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_DFCOULOMBCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_DFCOULOMBCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle                cuEST handle. Must not be NULL.\n @param[in]  plan                  Density-fitted integral plan (opaque handle) constructed with \\c cuestDFIntPlanCreate. Must not be NULL.\n @param[in]  parameters            Compute parameters (\\c cuestDFCoulombComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace    Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  densityMatrix         Input density matrix (size: nao × nao) on GPU. Must not be NULL.\n @param[out] outCoulombMatrix      Output buffer for computed Coulomb matrix (size: nao × nao) on GPU. Must not be NULL.\n                                   Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFCoulombCompute(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFCoulombComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    outCoulombMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestDFCoulombCompute
        .expect("cuEST function `cuestDFCoulombCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        outCoulombMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a DF Coulomb matrix computation.\n @ingroup Coulomb\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided \\c cuestDFIntPlan_t. Only sizes are computed; output and density\n buffers may be NULL.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_DFCOULOMBCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_DFCOULOMBCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle                              cuEST handle. Must not be NULL.\n @param[in]  plan                                Density-fitted integral plan (opaque handle). Must not be NULL.\n @param[in]  parameters                          Compute parameters (\\c cuestDFCoulombComputeParameters_t). Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor        Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      densityMatrix                       Optional density matrix (may be NULL).\n @param      outCoulombMatrix                    Optional output Coulomb matrix (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFCoulombComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFCoulombComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    outCoulombMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFCoulombComputeWorkspaceQuery.expect(
        "cuEST function `cuestDFCoulombComputeWorkspaceQuery` not found in the loaded library",
    )(handle, plan, parameters, temporaryWorkspaceDescriptor, densityMatrix, outCoulombMatrix)
}

#[doc = " @brief Compute the AO symmetric exchange (K) matrix using density fitting (DF) with the supplied DF integral plan and coefficient matrix.\n @ingroup Exchange\n\n - The input coefficient matrix must be preallocated to size (\\c numOccupied × nao) (row-major).\n - The output buffer \\c outExchangeMatrix must be preallocated for (nao × nao) doubles.\n - All handles, pointers, and buffers must be valid and non-NULL.\n - Temporary workspace must be preallocated using size returned by \\c cuestDFSymmetricExchangeComputeWorkspaceQuery.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        Density-fitted integral plan (opaque handle). Must not be NULL.\n @param[in]  parameters                  Compute parameters (\\c cuestDFSymmetricExchangeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for an internal scratch buffer (host and device). Must not be NULL.\n                                             For most problem sizes, 8 GB is a good buffer size. On older devices, such as A100,\n                                             smaller buffer sizes down to 2 GB can be used without degrading performance. Host memory\n                                             is currently unused. Note that the variableBufferSize does not limit total memory usage.\n                                             There may be static allocations required outside of the scope of what is limited by the\n                                             variableBufferSize. This value must remain unchanged between WorkspaceQuery and Compute calls.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrix           Input coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[out] outExchangeMatrix           Output buffer for the AO exchange matrix (size: nao × nao) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n\n The exchange matrix is computed as:\n     \\f[ K_{\\mu\\nu} = \\sum^{N_{ao}}_{\\lambda\\sigma} \\sum^{N_{occ}}_{i}\n                      \\left[ f_{\\rm HF} (\\mu\\lambda|\\frac{1}{r}|\\nu\\sigma) + f_{\\rm LRC} (\\mu\\lambda|\\frac{{\\rm erf}(\\omega r)}{r}|\\nu\\sigma) \\right]\n                      C_{i\\lambda} C_{i\\sigma} \\f]\n\n where \\f$ f_{\\rm HF} \\f$ is the fraction of Hartree-Fock exchange to add  (\\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION),\n \\f$ f_{\\rm LRC} \\f$ is the fraction of the long-range component of range-separated exchange to add (\\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION),\n and \\f$ \\omega \\f$ is the range-separation parameter.\n\n When the operator definition leads to zero exchange, the exchange matrix is still calculated and scaled by zero. The user should\n avoid calling this function in that case to avoid unnecessary compute.\n"]
pub unsafe fn cuestDFSymmetricExchangeCompute(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFSymmetricExchangeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outExchangeMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestDFSymmetricExchangeCompute
        .expect("cuEST function `cuestDFSymmetricExchangeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outExchangeMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a DF exchange matrix calculation.\n @ingroup Exchange\n\n This function returns (via output parameters) the host and device workspace buffer sizes needed to perform\n a symmetric exchange computation, given the plan and matrix dimensions. The coefficient matrix and output\n buffer may be NULL (only sizes are computed).\n\n @param[in]  handle                              cuEST handle. Must not be NULL.\n @param[in]  plan                                Density-fitted integral plan (opaque handle). Must not be NULL.\n @param[in]  parameters                          Compute parameters (\\c cuestDFSymmetricExchangeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize                  User-provided descriptor for an internal scratch buffer (host and device). Must not be NULL.\n                                                     For most problem sizes, 8 GB is a good buffer size. On older devices, such as A100,\n                                                     smaller buffer sizes down to 2 GB can be used without degrading performance. Host memory\n                                                     is currently unused. Note that the variableBufferSize does not limit total memory usage.\n                                                     There may be static allocations required outside of the scope of what is limited by the\n                                                     variableBufferSize. This value must remain unchanged between WorkspaceQuery and Compute calls.\n @param[out] temporaryWorkspaceDescriptor        Output descriptor for workspace sizes (host and device). Must not be NULL.\n @param[in]  numOccupied                         Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrix                   Optional pointer to coefficient matrix buffer (may be NULL).\n @param      outExchangeMatrix                   Optional pointer to output exchange matrix buffer (may be NULL).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFSymmetricExchangeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFSymmetricExchangeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outExchangeMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFSymmetricExchangeComputeWorkspaceQuery.expect("cuEST function `cuestDFSymmetricExchangeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, coefficientMatrix, outExchangeMatrix)
}

#[doc = " @brief Compute AO exchange (K) matrices with nonsymmetric densities using density fitting (DF) with the supplied DF integral plan and coefficient matrices.\n @ingroup Exchange\n @since v0.2.0\n\n - The input left coefficient matrix must be preallocated to size (\\c numOccupied × nao) (row-major).\n - The input right coefficient matrices must be preallocated to size (\\c numCoefficientMatrices × numOccupied × nao) (row-major).\n - The output buffer \\c outExchangeMatrices must be preallocated for (\\c numCoefficientMatrices × nao × nao) doubles.\n - All handles, pointers, and buffers must be valid and non-NULL.\n - Temporary workspace must be preallocated using size returned by \\c cuestDFNonsymmetricExchangeComputeWorkspaceQuery.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        Density-fitted integral plan (opaque handle). Must not be NULL.\n @param[in]  parameters                  Compute parameters (\\c cuestDFNonsymmetricExchangeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for an internal scratch buffer (host and device). Must not be NULL.\n                                             For most problem sizes, 8 GB is a good buffer size. On older devices, such as A100,\n                                             smaller buffer sizes down to 2 GB can be used without degrading performance. Host memory\n                                             is currently unused. Note that the variableBufferSize does not limit total memory usage.\n                                             There may be static allocations required outside of the scope of what is limited by the\n                                             variableBufferSize. This value must remain unchanged between WorkspaceQuery and Compute calls.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numCoefficientMatrices      Number of right coefficient matrices provided and output exchange matrices; must be greater than zero.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in left and right coefficient matrices); must be greater than zero.\n @param[in]  leftCoefficientMatrix       Input left coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[in]  rightCoefficientMatrices    Input right coefficient matrices (size: \\c numCoefficientMatrices × numOccupied × nao) on GPU. Must not be NULL.\n @param[out] outExchangeMatrices         Output buffer for the AO exchange matrices (size: numCoefficientMatrices × nao × nao) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied or \\c numCoefficientMatrices is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n\n The exchange matrix is computed as:\n     \\f[ K^{n}_{\\mu\\nu} = \\sum^{N_{ao}}_{\\lambda\\sigma} \\sum^{N_{occ}}_{i}\n                      \\left[ f_{\\rm HF} (\\mu\\lambda|\\frac{1}{r}|\\nu\\sigma) + f_{\\rm LRC} (\\mu\\lambda|\\frac{{\\rm erf}(\\omega r)}{r}|\\nu\\sigma) \\right]\n                      L_{i\\lambda} R^{n}_{i\\sigma} \\f]\n\n where \\f$ f_{\\rm HF} \\f$ is the fraction of Hartree-Fock exchange to add  (\\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION),\n \\f$ f_{\\rm LRC} \\f$ is the fraction of the long-range component of range-separated exchange to add (\\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION),\n and \\f$ \\omega \\f$ is the range-separation parameter.\n\n When the operator definition leads to zero exchange, the exchange matrix is still calculated and scaled by zero. The user should\n avoid calling this function in that case to avoid unnecessary compute."]
pub unsafe fn cuestDFNonsymmetricExchangeCompute(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFNonsymmetricExchangeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numCoefficientMatrices: u64,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrices: *const f64,
    outExchangeMatrices: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFNonsymmetricExchangeCompute.expect(
        "cuEST function `cuestDFNonsymmetricExchangeCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numCoefficientMatrices,
        numOccupied,
        leftCoefficientMatrix,
        rightCoefficientMatrices,
        outExchangeMatrices,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for the calculation of nonsymmetric DF exchange matrices.\n @ingroup Exchange\n @since v0.2.0\n\n This function returns (via output parameters) the host and device workspace buffer sizes needed to perform\n a nonsymmetric exchange computation, given the plan and matrix dimensions. The coefficient matrices and output\n buffer may be NULL (only sizes are computed).\n\n @param[in]  handle                         cuEST handle. Must not be NULL.\n @param[in]  plan                           Density-fitted integral plan (opaque handle). Must not be NULL.\n @param[in]  parameters                     Compute parameters (\\c cuestDFNonsymmetricExchangeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize             User-provided descriptor for an internal scratch buffer (host and device). Must not be NULL.\n                                                For most problem sizes, 8 GB is a good buffer size. On older devices, such as A100,\n                                                smaller buffer sizes down to 2 GB can be used without degrading performance. Host memory\n                                                is currently unused. Note that the variableBufferSize does not limit total memory usage.\n                                                There may be static allocations required outside of the scope of what is limited by the\n                                                variableBufferSize. This value must remain unchanged between WorkspaceQuery and Compute calls.\n @param[out] temporaryWorkspaceDescriptor   Output descriptor for workspace sizes (host and device). Must not be NULL.\n @param[in]  numCoefficientMatrices         Number of right coefficient matrices provided and output exchange matrices; must be greater than zero.\n @param[in]  numOccupied                    Number of occupied orbitals (rows in left and right coefficient matrices); must be greater than zero.\n @param      leftCoefficientMatrix          Input left coefficient matrix (size: \\c numOccupied × nao) on GPU. May be NULL.\n @param      rightCoefficientMatrices       Input right coefficient matrices (size: \\c numCoefficientMatrices × numOccupied × nao) on GPU. May be NULL.\n @param      outExchangeMatrices            Output buffer for the AO exchange matrices (size: numCoefficientMatrices × nao × nao) on GPU. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied or \\c numCoefficientMatrices is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFNonsymmetricExchangeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFNonsymmetricExchangeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numCoefficientMatrices: u64,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrices: *const f64,
    outExchangeMatrices: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFNonsymmetricExchangeComputeWorkspaceQuery.expect("cuEST function `cuestDFNonsymmetricExchangeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numCoefficientMatrices, numOccupied, leftCoefficientMatrix, rightCoefficientMatrices, outExchangeMatrices)
}

#[doc = " @brief Compute the nuclear gradient of the symmetric DF J/K matrices contracted\n        with a density matrix and multiple coefficient matrices.\n @ingroup JKGrad\n\n This routine evaluates the nuclear derivatives (gradients) of the Coulomb (J) and\n exchange (K) matrices in density fitting approximation, where the J matrix is\n contracted with a single density matrix and the K matrices are contracted with\n multiple occupied orbital coefficient matrices. The result overwrites the\n output gradient buffer (size: natom × 3).\n\n All required temporary workspace must be sized using the corresponding workspace\n query function. A \\c variableBufferSize descriptor must also be provided to\n constrain the memory usage.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         DF integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestDFSymmetricDerivativeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize           The variableBufferSize determines the size of an internal scratch buffer\n                                              used for certain transformations. A buffer of 2GB is a reasonable default value.\n                                              Host memory is currently unused. Note that the variableBufferSize does not limit total\n                                              memory usage. The value of the variableBufferSize must not change between WorkspaceQuery\n                                              and Compute calls.\n @param[in]  temporaryWorkspace           Temporary workspace buffers (preallocated for\n                                              this operation using the query function).\n                                              Must not be NULL.\n @param[in]  densityScale                 Scaling factor for the J (Coulomb) contribution.\n @param[in]  densityMatrix                Input density matrix (size: nao × nao) on GPU.\n                                              Must not be NULL.\n @param[in]  coefficientScale             Scaling factor for each K (exchange) contribution.\n @param[in]  numCoefficientMatrices       Number of coefficient matrices. Must be > 0 when exchange is active; may be 0 for pure functionals.\n @param[in]  numOccupied                  Array of occupied orbital counts for each\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0 when exchange is active; for pure functionals the array may be NULL or contain zeros.\n @param[in]  coefficientMatrices          Concatenated coefficient matrices. Each matrix\n                                              has shape [\\c numOccupied[i], nao] on GPU. Total size:\n                                              sum(\\c numOccupied[i]) × nao. Must not be NULL when exchange\n                                              is active; may be NULL for pure functionals (if provided, it is ignored).\n @param[out] outGradient                  Output nuclear gradient (size: natom × 3) on GPU.\n                                              Gradient results overwrite this buffer.\n                                              Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c coefficientScale != 0.0 and\n             if \\c numCoefficientMatrices == 0 or any \\c numOccupied[i] == 0;\n             or if \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION or \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION\n             != 0.0 and\n             if \\c numCoefficientMatrices == 0 or any \\c numOccupied[i] == 0;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n\n The bare Coulomb and exchange energies are given as:\n\n \\f[ E_{J}     = \\sum_{\\mu\\nu\\lambda\\sigma} D_{\\mu\\nu} (\\mu\\nu|\\lambda\\sigma) D_{\\lambda\\sigma} \\f]\n \\f[ E_{K}^{N} = \\sum_{\\mu\\nu\\lambda\\sigma} \\sum_{ij}\n                 \\left[ f_{\\rm HF} (\\mu\\lambda|\\frac{1}{r}|\\nu\\sigma) + f_{\\rm LRC} (\\mu\\lambda|\\frac{{\\rm erf}(\\omega r)}{r}|\\nu\\sigma) \\right]\n                 C_{i\\mu}^{N} C_{i\\nu}^{N} C_{j\\lambda}^{N} C_{j\\sigma}^{N} \\f]\n\n An effective J/K energy can be written as:\n\n \\f[ E_{JK} = s_D * E_{J} + \\sum_{N} s_C * E_{K}^{N} \\f]\n\n where\n   - \\f$ s_D \\f$ is a density scaling factor \\c densityScale,\n   - \\f$ s_C \\f$ is a coefficient scaling factor \\c coefficientScale,\n   - \\f$ f_{\\rm HF} \\f$ is the fraction of Hartree-Fock exchange to add, \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION\n   - \\f$ f_{\\rm LRC} \\f$ is the fraction of long-range range-separated exchange to add, \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION\n   - \\f$ \\omega \\f$ is the range-separation parameter to use, \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA\n\n This function computes: \\f$dE_{JK}/dR\\f$, where R represents the basis function centers of the electron\n repulsion integrals.\n\n \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION, \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION, and \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA\n are set when the \\c cuestDFIntPlan_t is constructed and is intended to\n describe the exchange parameterization in hybrid and range-separated hybrid functionals.\n\n The \\c densityScale and \\c coefficientScale parameters are intended to account for the spin summations needed to obtain\n the correct two-electron energy.\n\n For example, in a B3LYP RKS computation the intended usage would be:\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION = 0.2\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION = 0.0\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA unused\n   - \\c densityScale = 2.0\n   - \\c coefficientScale = -1.0\n   - \\c numCoefficientMatrices = 1\n   - \\f$ D_{mn} = \\sum_{i} C_{im} C_{in} \\f$\n   - \\f$ I_{ij} = \\sum_{mn} C_{im} S_{mn} C_{jn} \\f$\n\n For a PBE0 UKS computation the intended usage would be:\n   - \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION = 0.25\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION = 0.0\n   - \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA unused\n   - \\c densityScale = 0.5\n   - \\c coefficientScale = -0.5\n   - \\c numCoefficientMatrices = 2\n   -  \\f$D_{mn} = \\sum_{i} C_{im}^{\\alpha} C_{in}^{\\alpha} + \\sum_{i} C_{im}^{\\beta} C_{in}^{\\beta} \\f$\n   -  \\f$I_{ij} = \\sum_{mn} C_{im}^{\\alpha} S_{mn} C_{jn}^{\\alpha} \\f$\n   -  \\f$I_{ij} = \\sum_{mn} C_{im}^{\\beta} S_{mn} C_{jn}^{\\beta} \\f$\n\n For pure functionals, where \\c coefficientScale = 0.0 (\\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION\n and \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION are zero),\n the coefficient matrices do not need to be provided (if they are, they will not be used). For the pure\n functional case, the \\c numCoefficientMatrices can be 0. The \\c numOccupied array can be NULL (or contain zeros).\n The \\c coefficientMatrices array can be NULL.\n\n The following memory usage considerations apply only to cases where the fraction of Hartree-Fock exchange is non-zero.\n\n The memory usage during the gradient evaluation can be changed at a high level by CUEST_DFSYMMETRICDERIVATIVECOMPUTE_PARAMETERS_MEMORY_POLICY.\n - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_MEMORY_POLICY_DEVICECACHE: Default. This will provide good performance, but may try to allocate a large\n     amount of GPU memory.\n - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_MEMORY_POLICY_HOSTCACHE: Provides similar performance, but will allocate that memory on the CPU,\n     incurring a bit of overhead transferring memory to/from the CPU.\n - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_MEMORY_POLICY_OVERWRITE: Preferred, where possible.\n     This memory policy avoids any additional large allocations and/or host/device memory transfers by overwriting some of the temporary data in\n     the \\c cuestDFIntPlan_t. This corrupts the \\c cuestDFIntPlan_t and should be destroyed following this call.\n     If the gradient evaluation is the final function call\n     using the \\c cuestDFIntPlan_t, then this is optimal (this is often the case in usual HF/DFT gradient calculations).\n     If the \\c cuestDFIntPlan_t will be needed again, then this option is not suitable.\n - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_MEMORY_POLICY_RECOMPUTE: This memory policy does not require any additional allocations or memory transfers.\n     It corrupts the data in the \\c cuestDFIntPlan_t during the gradient calculation. However, following the gradient call, it returns the\n     \\c cuestDFIntPlan_t to a usable state by recomputing (and replacing) any corrupted data. This may add significantly to the cost of the\n     gradient evaluation.\n\n Note that variableBufferSize only applies to the formation of certain intermediates during the cuestDFSymmetricDerivativeCompute and will not\n limit overall memory usage. We recommend a buffer size of at least 2 GB with modest performance improvements possible if larger buffers can be provided.\n"]
pub unsafe fn cuestDFSymmetricDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFSymmetricDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityScale: f64,
    densityMatrix: *const f64,
    coefficientScale: f64,
    numCoefficientMatrices: u64,
    numOccupied: *const u64,
    coefficientMatrices: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFSymmetricDerivativeCompute.expect(
        "cuEST function `cuestDFSymmetricDerivativeCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        densityScale,
        densityMatrix,
        coefficientScale,
        numCoefficientMatrices,
        numOccupied,
        coefficientMatrices,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required for a DF J/K nuclear gradient computation.\n @ingroup JKGrad\n\n This function determines the workspace required for a symmetric DF J/K gradient\n evaluation with the given plan, density matrix, coefficient matrices, and maximum\n workspace constraint. The output descriptor is filled on success and can be used\n to allocate host and device workspace buffers prior to calling\n \\c cuestDFSymmetricDerivativeCompute.\n\n Some input pointers (\\c densityMatrix, \\c coefficientMatrices, \\c outGradient) are optional\n and may be NULL.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_DFSYMMETRICDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is expected to provide\n     a performance advantage on the present GPU architecture.\n   - ffloat compute is only compatible with JIT-compiled kernels. Requesting ffloat compute while JIT is\n     disabled (\\c CUEST_FFLOAT_USAGE_MODE_ON together with \\c CUEST_JIT_USAGE_MODE_OFF) is not supported and\n     returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         DF integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestDFSymmetricDerivativeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize           The variableBufferSize determines the size of an internal scratch buffer\n                                              used for certain transformations. A buffer of 2GB is a reasonable default value.\n                                              Host memory is currently unused. Note that the variableBufferSize does not limit total\n                                              memory usage. The value of the variableBufferSize must not change between WorkspaceQuery\n                                              and Compute calls.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required temporary workspace\n                                              sizes (host and device). Must not be NULL.\n @param[in]  densityScale                 Scaling factor for the J (Coulomb) contribution.\n @param      densityMatrix                Optional density matrix. May be NULL.\n @param[in]  coefficientScale             Scaling factor for each K (exchange) contribution.\n @param[in]  numCoefficientMatrices       Number of coefficient matrices. Must be > 0 when exchange is active; may be 0 for pure functionals.\n @param[in]  numOccupied                  Array of occupied orbital counts for each\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0 when exchange is active; for pure functionals the array may be NULL or contain zeros.\n @param      coefficientMatrices          Optional concatenated coefficient matrices. May be NULL.\n @param      outGradient                  Optional output gradient buffer. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c coefficientScale != 0.0 and\n             if \\c numCoefficientMatrices == 0 or any \\c numOccupied[i] == 0;\n             or if \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION or \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION\n             != 0.0 and\n             if \\c numCoefficientMatrices == 0 or any \\c numOccupied[i] == 0;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package, or if ffloat compute is requested while JIT is disabled (incompatible\n     JIT/ffloat combination);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFSymmetricDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFSymmetricDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityScale: f64,
    densityMatrix: *const f64,
    coefficientScale: f64,
    numCoefficientMatrices: u64,
    numOccupied: *const u64,
    coefficientMatrices: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFSymmetricDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestDFSymmetricDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, densityScale, densityMatrix, coefficientScale, numCoefficientMatrices, numOccupied, coefficientMatrices, outGradient)
}

#[doc = " @brief Transforms the 3-index density fitting tensor from the AO basis to the MO basis.\n @ingroup DFMOIntegrals\n @since v0.2.0\n\n This routine transforms three-index AO integrals into the MO basis within the density\n fitting representation. Pairs of coefficient matrices are taken as input.\n\n Starting from a density-fitted representation of the AO integrals:\n\n \\f[\n \\left[ f_{\\rm HF} (\\mu\\nu|\\frac{1}{r}|\\lambda\\sigma) + f_{\\rm LRC} (\\mu\\nu|\\frac{{\\rm erf}(\\omega r)}{r}|\\lambda\\sigma) \\right]\n     = \\sum_{A} B^{A}_{\\mu\\nu} B^{A}_{\\lambda\\sigma}\n \\f]\n\n This function performs the transformation:\n\n \\f[ B^{A}_{ia} = \\sum_{\\mu\\nu} B^{A}_{\\mu\\nu} C(L)_{i\\mu} C(R)_{a\\nu} \\f]\n\n The resulting tensors can be used to directly construct 4-index MO integrals by contracting over the auxiliary index.\n\n \\f[\n \\left[ f_{\\rm HF} (ia|\\frac{1}{r}|jb) + f_{\\rm LRC} (ia|\\frac{{\\rm erf}(\\omega r)}{r}|jb) \\right]\n     = \\sum_{A} B^{A}_{ia} B^{A}_{jb}\n \\f]\n\n The transformation is most efficient when the MO dimension of C(L) is\n smaller than the MO dimension of C(R), as C(L) is applied first. The\n resulting tensor is transformed by the fitting metric.\n\n Multiple MO integral tensors can be evaluated in a single function call.\n The resulting MO integral tensors respect the values of\n \\c CUEST_DFINTPLAN_PARAMETERS_EXCHANGE_FRACTION,\n \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_FRACTION, and\n \\c CUEST_DFINTPLAN_PARAMETERS_LRC_EXCHANGE_OMEGA set in the \\c cuestDFIntPlan_t.\n\n This function supports the use of a \\c cuestDFIntPlan_t constructed with the\n CUEST_DFINTPLAN_PARAMETERS_THREE_INDEX_INTEGRAL_DIRECT parameter set to either\n 0 or 1.\n\n All required temporary workspace must be sized using the corresponding workspace\n query function. A \\c variableBufferSize descriptor must also be provided to\n constrain the memory usage.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         DF integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestDFMOIntegralsComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize           The variableBufferSize determines the size of an internal scratch buffer\n                                              used for certain transformations. A buffer of 2GB is a reasonable default value.\n                                              Host memory is currently unused. Note that the variableBufferSize does not limit total\n                                              memory usage. The value of the variableBufferSize must not change between WorkspaceQuery\n                                              and Compute calls.\n @param[in]  temporaryWorkspace           Temporary workspace buffers (preallocated for\n                                              this operation using the query function).\n                                              Must not be NULL.\n @param[in]  numCoefficientMatrices       Number of pairs of coefficient matrices. Must be > 0.\n @param[in]  numLeftOrbitals              Array of molecular orbital counts for each left\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0. Must not be NULL.\n @param[in]  numRightOrbitals             Array of molecular orbital counts for each right\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0. Must not be NULL.\n @param[in]  leftCoefficientMatrices      Concatenated left coefficient matrices. Each matrix\n                                              has shape [\\c numLeftOrbitals[i], nao] on GPU. Total size:\n                                              sum(\\c numLeftOrbitals[i]) × nao. Must not be NULL.\n @param[in]  rightCoefficientMatrices     Concatenated right coefficient matrices. Each matrix\n                                              has shape [\\c numRightOrbitals[i], nao] on GPU. Total size:\n                                              sum(\\c numRightOrbitals[i]) × nao. Must not be NULL.\n @param[out] outTensors                   Output concatenated MO integral tensors. Each tensor has shape\n                                              [naux, \\c numLeftOrbitals[i], \\c numRightOrbitals[i]] on GPU.\n                                              Total size: naux × sum(\\c numRightOrbitals[i] × \\c numLeftOrbitals[i]).\n                                              Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT\n             if \\c numCoefficientMatrices == 0,\n             or any \\c numLeftOrbitals[i] == 0,\n             or any \\c numRightOrbitals[i] == 0;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n"]
pub unsafe fn cuestDFMOIntegralsCompute(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFMOIntegralsComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numCoefficientMatrices: u64,
    numLeftOrbitals: *const u64,
    numRightOrbitals: *const u64,
    leftCoefficientMatrices: *const f64,
    rightCoefficientMatrices: *const f64,
    outTensors: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestDFMOIntegralsCompute
        .expect("cuEST function `cuestDFMOIntegralsCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numCoefficientMatrices,
        numLeftOrbitals,
        numRightOrbitals,
        leftCoefficientMatrices,
        rightCoefficientMatrices,
        outTensors,
    )
}

#[doc = " @brief Query the temporary workspace required to transform the 3-index density\n     fitting tensor from the AO basis to the MO basis.\n @ingroup DFMOIntegrals\n @since v0.2.0\n\n This function determines the workspace required to perform an AO to MO transform\n of the DF 3-index tensor with the given plan, coefficient matrices, and maximum\n workspace constraint. The output descriptor is filled on success and can be used\n to allocate host and device workspace buffers prior to calling\n \\c cuestDFMOIntegralsCompute.\n\n Some input pointers (\\c coefficientMatrices and \\c outTensors) are optional\n and may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         DF integral computation plan (opaque handle). Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestDFMOIntegralsComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize           The variableBufferSize determines the size of an internal scratch buffer\n                                              used for certain transformations. A buffer of 2GB is a reasonable default value.\n                                              Host memory is currently unused. Note that the variableBufferSize does not limit total\n                                              memory usage. The value of the variableBufferSize must not change between WorkspaceQuery\n                                              and Compute calls.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required temporary workspace\n                                              sizes (host and device). Must not be NULL.\n @param[in]  numCoefficientMatrices       Number of pairs of coefficient matrices. Must be > 0.\n @param[in]  numLeftOrbitals              Array of molecular orbital counts for each left\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0. Must not be NULL.\n @param[in]  numRightOrbitals             Array of molecular orbital counts for each right\n                                              coefficient matrix (size: \\c numCoefficientMatrices) on CPU.\n                                              Each entry must be > 0. Must not be NULL.\n @param      leftCoefficientMatrices      Concatenated left coefficient matrices. Each matrix\n                                              has shape [\\c numLeftOrbitals[i], nao] on GPU. Total size:\n                                              sum(\\c numLeftOrbitals[i]) × nao. May be NULL.\n @param      rightCoefficientMatrices     Concatenated right coefficient matrices. Each matrix\n                                              has shape [\\c numRightOrbitals[i], nao] on GPU. Total size:\n                                              sum(\\c numRightOrbitals[i]) × nao. May be NULL.\n @param      outTensors                   Output concatenated MO integral tensors. Each tensor has shape\n                                              [naux, \\c numLeftOrbitals[i], \\c numRightOrbitals[i]] on GPU.\n                                              Total size: naux × sum(\\c numRightOrbitals[i] × \\c numLeftOrbitals[i]).\n                                              May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_PLAN if the \\c cuestDFIntPlan_t is not correctly configured or in a dirty state;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT\n             if \\c numCoefficientMatrices == 0,\n             or any \\c numLeftOrbitals[i] == 0,\n             or any \\c numRightOrbitals[i] == 0;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestDFMOIntegralsComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestDFIntPlan_t,
    parameters: cuestDFMOIntegralsComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numCoefficientMatrices: u64,
    numLeftOrbitals: *const u64,
    numRightOrbitals: *const u64,
    leftCoefficientMatrices: *const f64,
    rightCoefficientMatrices: *const f64,
    outTensors: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestDFMOIntegralsComputeWorkspaceQuery.expect(
        "cuEST function `cuestDFMOIntegralsComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        numCoefficientMatrices,
        numLeftOrbitals,
        numRightOrbitals,
        leftCoefficientMatrices,
        rightCoefficientMatrices,
        outTensors,
    )
}

#[doc = " @brief Create an atom grid object with specified parameters.\n @ingroup Grid\n\n @note The \\c radialWeights positivity and \\c numAngularPoints Lebedev-number requirements are not\n       validated by this function; they are enforced when this atom grid is used to construct a\n       \\c cuestMolecularGrid_t, where a violation is reported as \\c CUEST_STATUS_EXCEPTION.\n\n @param[in]  handle            cuEST handle. Must not be NULL.\n @param[in]  numRadialPoints   Number of radial points. Must be greater than zero.\n @param[in]  radialNodes       Location of radial nodes from the atom center (size: \\c numRadialPoints) on the CPU. Must not be NULL.\n @param[in]  radialWeights     Pointer to array containing quadrature weights of a radial point (size: \\c numRadialPoints) on the CPU. Elements must be positive semidefinite. Must not be NULL.\n @param[in]  numAngularPoints  Pointer to array containing number of angular points associated with a radial point (size: \\c numRadialPoints) on the CPU. Values must be Lebedev numbers. Must not be NULL.\n @param[in]  parameters        Handle to \\c cuestAtomGridParameters_t. Must not be NULL.\n @param[out] outAtomGrid       Pointer to output atom grid object; receives opaque handle on success. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle or parameters are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numRadialPoints is zero;\n   - \\c CUEST_STATUS_EXCEPTION for unexpected error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR for non-std exceptions."]
pub unsafe fn cuestAtomGridCreate(
    handle: cuestHandle_t,
    numRadialPoints: u64,
    radialNodes: *const f64,
    radialWeights: *const f64,
    numAngularPoints: *const u64,
    parameters: cuestAtomGridParameters_t,
    outAtomGrid: *mut cuestAtomGrid_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestAtomGridCreate
        .expect("cuEST function `cuestAtomGridCreate` not found in the loaded library")(
        handle,
        numRadialPoints,
        radialNodes,
        radialWeights,
        numAngularPoints,
        parameters,
        outAtomGrid,
    )
}

#[doc = " @brief Destroy/free an atom grid object created with \\c cuestAtomGridCreate.\n @ingroup Grid\n\n Frees all memory associated with the atom grid. After destruction, the grid handle must not be used.\n\n @param[in] atomGrid   Opaque atom grid handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c atomGrid is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c atomGrid is not of type \\c cuestAtomGrid_t;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestAtomGridDestroy(atomGrid: cuestAtomGrid_t) -> cuestStatus_t {
    dyload_lib()
        .cuestAtomGridDestroy
        .expect("cuEST function `cuestAtomGridDestroy` not found in the loaded library")(
        atomGrid
    )
}

#[doc = " @brief Create a molecular grid object.\n @ingroup Grid\n\n The molecular grid comprises multiple atom grids each at an atom center.\n - All buffer pointers must be valid (not NULL).\n - Each atom grid, and each array entry, must be valid.\n - The persistent and temporary workspaces must be provided and preallocated.\n - The output handle is only valid after successful creation.\n\n @param[in] handle                cuEST handle. Must not be NULL.\n @param[in] numAtoms              Total number of atoms; must be greater than zero.\n @param[in] atomGrid              Array of atom grid handles (size: \\c numAtoms). CPU array, must not be NULL and no elements of the array may be NULL.\n @param[in] xyz                   Array of atomic Cartesian coordinates (size: \\c numAtoms * 3) on the CPU. Ordering of elements is row-major: \\f$ [x_0, y_0, z_0, ...] \\f$. Must not be NULL.\n @param[in] parameters            Handle to \\c cuestMolecularGridParameters_t. Must not be NULL.\n @param[in] persistentWorkspace   Pointer to preallocated workspace for persistent buffers. Must not be NULL.\n @param[in] temporaryWorkspace    Pointer to preallocated workspace for temporary buffers. Must not be NULL.\n @param[out] outGrid              Pointer for output molecular grid. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer parameter is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle, parameters or atom grids are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestMolecularGridCreate(
    handle: cuestHandle_t,
    numAtoms: u64,
    atomGrid: *const cuestAtomGrid_t,
    xyz: *const f64,
    parameters: cuestMolecularGridParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outGrid: *mut cuestMolecularGrid_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestMolecularGridCreate
        .expect("cuEST function `cuestMolecularGridCreate` not found in the loaded library")(
        handle,
        numAtoms,
        atomGrid,
        xyz,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outGrid,
    )
}

#[doc = " @brief Query workspace requirements for creating a molecular grid object.\n @ingroup Grid\n\n This function simulates the allocation logic to determine necessary persistent and temporary buffer sizes.\n All buffer pointers and arrays must be valid.\n\n @param[in]  handle                               cuEST handle. Must not be NULL.\n @param[in]  numAtoms                             Total number of atoms; must be greater than zero.\n @param[in]  atomGrid                             Flat array of atom grid handles (size: \\c numAtoms). CPU array, must not be NULL and no elements of the array may be NULL.\n @param[in]  xyz                                  Array of atomic Cartesian coordinates (size: \\c numAtoms * 3) on the CPU. Ordering of elements is row-major: \\f$ [x_0, y_0, z_0, ...] \\f$. Must not be NULL.\n @param[in]  parameters                           Handle to \\c cuestMolecularGridParameters_t. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor        Output descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor         Output descriptor for temporary buffer sizing. Must not be NULL.\n @param      outGrid                              Pointer for output molecular grid. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the cuEST handle, parameters or atom grids are not of the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numAtoms is zero;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for other errors."]
pub unsafe fn cuestMolecularGridCreateWorkspaceQuery(
    handle: cuestHandle_t,
    numAtoms: u64,
    atomGrid: *const cuestAtomGrid_t,
    xyz: *const f64,
    parameters: cuestMolecularGridParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outGrid: *mut cuestMolecularGrid_t,
) -> cuestStatus_t {
    dyload_lib().cuestMolecularGridCreateWorkspaceQuery.expect(
        "cuEST function `cuestMolecularGridCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        numAtoms,
        atomGrid,
        xyz,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outGrid,
    )
}

#[doc = " @brief Destroy/free a molecular grid object and all associated resources.\n @ingroup Grid\n\n After this function is called, the grid handle must not be used.\n\n @param[in] grid    Molecular grid handle to destroy; must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c grid is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c grid is not of type \\c cuestMolecularGrid_t;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR on error."]
pub unsafe fn cuestMolecularGridDestroy(grid: cuestMolecularGrid_t) -> cuestStatus_t {
    dyload_lib()
        .cuestMolecularGridDestroy
        .expect("cuEST function `cuestMolecularGridDestroy` not found in the loaded library")(
        grid
    )
}

#[doc = " @brief Build a plan for exchange-correlation integral evaluation.\n @ingroup XCIntegralPlan\n\n All object handles, workspaces, and output pointers must be valid (not NULL).\n Workspace buffers must be preallocated with required sizes (see workspace query).\n\n \\verbatim embed:rst\n.. Note::\n- The number of atoms in the \\c basis must match the number of atoms in the \\c grid.\n\\endverbatim\n\n @param[in]  handle               cuEST handle. Must not be NULL.\n @param[in]  basis                AO basis set handle. Must not be NULL.\n @param[in]  grid                 Molecular grid handle. Must not be NULL.\n @param[in]  functional           Specifies the XC functional.\n @param[in]  parameters           Plan parameter structure. Must not be NULL.\n @param[in]  persistentWorkspace  Preallocated workspace for persistent buffers. Must not be NULL.\n @param[in]  temporaryWorkspace   Preallocated workspace for temporary buffers. Must not be NULL.\n @param[out] outPlan              Output pointer for the XC integral plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c functional does not exist;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntPlanCreate(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    grid: cuestMolecularGrid_t,
    functional: cuestXCIntPlanParametersFunctional_t,
    parameters: cuestXCIntPlanParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outPlan: *mut cuestXCIntPlan_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCIntPlanCreate
        .expect("cuEST function `cuestXCIntPlanCreate` not found in the loaded library")(
        handle,
        basis,
        grid,
        functional,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        outPlan,
    )
}

#[doc = " @brief Query workspace requirements for creating an exchange-correlation integral plan.\n @ingroup XCIntegralPlan\n\n Determines persistent and temporary buffer sizes by simulating allocation logic.\n All handles and pointers must be valid (not NULL).\n\n @param[in]  handle                        cuEST handle. Must not be NULL.\n @param[in]  basis                         AO basis set handle. Must not be NULL.\n @param[in]  grid                          Molecular grid handle. Must not be NULL.\n @param[in]  functional                    Specifies the XC functional.\n @param[in]  parameters                    Plan parameters handle. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor Output descriptor for persistent buffer sizing. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor  Output descriptor for temporary buffer sizing. Must not be NULL.\n @param      outPlan                       Output pointer for the XC integral plan handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c functional does not exist;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntPlanCreateWorkspaceQuery(
    handle: cuestHandle_t,
    basis: cuestAOBasis_t,
    grid: cuestMolecularGrid_t,
    functional: cuestXCIntPlanParametersFunctional_t,
    parameters: cuestXCIntPlanParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outPlan: *mut cuestXCIntPlan_t,
) -> cuestStatus_t {
    dyload_lib().cuestXCIntPlanCreateWorkspaceQuery.expect(
        "cuEST function `cuestXCIntPlanCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        basis,
        grid,
        functional,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        outPlan,
    )
}

#[doc = " @brief Destroy and release an exchange-correlation integral plan and its resources.\n @ingroup XCIntegralPlan\n\n The plan handle and its memory must not be used after destruction.\n\n @param[in] plan   Plan handle to destroy. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c plan is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntPlanDestroy(plan: cuestXCIntPlan_t) -> cuestStatus_t {
    dyload_lib()
        .cuestXCIntPlanDestroy
        .expect("cuEST function `cuestXCIntPlanDestroy` not found in the loaded library")(plan)
}

#[doc = " @brief Compute molecular integration grid point coordinates.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine retrieves the Cartesian coordinates of each molecular grid point\n used for exchange–correlation (XC) integration. These are the grid points used\n by the corresponding XC integral plan.\n\n The resulting coordinates are written to a user-provided output buffer\n of shape (\\c npoint × 3), containing the x, y, z coordinates of each grid point.\n\n Temporary device workspace must be preallocated based on the size returned by\n \\c cuestXCIntegrationGridComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestXCIntegrationGridComputeParameters_t) Must not be NULL\n @param[in]  temporaryWorkspace Preallocated GPU workspace buffer. Must not be NULL.\n @param[out] outGridPoints      Output buffer for computed grid point coordinates (size: \\c npoint × 3) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle or plan type is incorrect;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntegrationGridCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCIntegrationGridComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outGridPoints: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCIntegrationGridCompute
        .expect("cuEST function `cuestXCIntegrationGridCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        outGridPoints,
    )
}

#[doc = " @brief Query temporary workspace requirements for grid coordinate computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines the GPU memory workspace needed to generate the coordinates\n of molecular integration grid points defined by the provided XC integral plan.\n This function must be called before \\c cuestXCIntegrationGridCompute to ensure sufficient allocation.\n\n Only workspace sizes are computed; input and output pointers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestXCIntegrationGridComputeParameters_t) Must not be NULL\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required workspace sizes (in bytes). Must not be NULL.\n @param      outGridPoints                Output array for grid point coordinates. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handles are of incorrect type;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntegrationGridComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCIntegrationGridComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outGridPoints: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCIntegrationGridComputeWorkspaceQuery.expect("cuEST function `cuestXCIntegrationGridComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, outGridPoints)
}

#[doc = " @brief Compute molecular integration weights for XC quadrature.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine computes the integration weights associated with each grid point\n in the molecular integration grid used for exchange–correlation (XC) integration.\n The weights returned depend on the selected weight type, specified by \\c weightType.\n\n Supported weight types include:\n   - \\c CUEST_XCINTEGRATIONWEIGHT_PARAMETERS_WEIGHTTYPE_TOTAL — total integration weight (angular x radial x Becke);\n   - \\c CUEST_XCINTEGRATIONWEIGHT_PARAMETERS_WEIGHTTYPE_BECKE — Becke atomic partition weights;\n   - \\c CUEST_XCINTEGRATIONWEIGHT_PARAMETERS_WEIGHTTYPE_QUADRATURE — quadrature-only weights (angular x radial).\n\n The result is written to \\c outGridWeights (size: \\c npoint × 1).\n Temporary workspace must be preallocated based on\n \\c cuestXCIntegrationWeightComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  weightType         Weight type to return (total, Becke, quadrature).\n @param[in]  parameters         Compute parameters (\\c cuestXCIntegrationWeightComputeParameters_t) Must not be NULL\n @param[in]  temporaryWorkspace Preallocated GPU workspace buffer for temporary data. Must not be NULL.\n @param[out] outGridWeights     Output buffer for integration weights (size: \\c npoint × 1) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle or plan type is invalid;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c weightType is unrecognized;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntegrationWeightCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    weightType: cuestXCIntegrationWeightComputeParametersWeightType_t,
    parameters: cuestXCIntegrationWeightComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outGridWeights: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCIntegrationWeightCompute
        .expect("cuEST function `cuestXCIntegrationWeightCompute` not found in the loaded library")(
        handle,
        plan,
        weightType,
        parameters,
        temporaryWorkspace,
        outGridWeights,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for computing XC integration weights.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines GPU memory workspace requirements to compute integration weights\n on the molecular grid as defined by the provided XC integral plan. The required\n workspace depends on the grid structure and selected weight type.\n\n This query should be executed before calling \\c cuestXCIntegrationWeightCompute\n to ensure that sufficient device memory is available.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  weightType                   Weight type to return (total, Becke, quadrature).\n @param[in]  parameters                   Compute parameters (\\c cuestXCIntegrationWeightComputeParameters_t) Must not be NULL\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required temporary workspace sizes (in bytes). Must not be NULL.\n @param      outGridWeights               Optional output buffer for computed weights. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle or plan type is incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c weightType is unsupported;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCIntegrationWeightComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    weightType: cuestXCIntegrationWeightComputeParametersWeightType_t,
    parameters: cuestXCIntegrationWeightComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outGridWeights: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCIntegrationWeightComputeWorkspaceQuery.expect("cuEST function `cuestXCIntegrationWeightComputeWorkspaceQuery` not found in the loaded library")(handle, plan, weightType, parameters, temporaryWorkspaceDescriptor, outGridWeights)
}

#[doc = " @brief Compute the RKS XC potential (Vxc) matrix using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup ExchangeCorrelation\n\n This routine computes the XC potential matrix (Vxc) from occupied molecular orbitals.\n This is the RKS interface that assumes the alpha and beta spin orbitals are identical. The user must\n provide occupied molecular orbitals (size nocc x nao) and a preallocated output buffer (size nao × nao).\n The orbital basis and integration grid used are passed via the \\c plan. All required temporary workspace\n must be allocated using the size returned by \\c cuestXCPotentialRKSComputeWorkspaceQuery.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                             The cuestXCIntPlanParametersFunctional_t set during plan construction determines what XC potential is evaluated.\n @param[in]  parameters                  Compute parameters (\\c cuestXCPotentialRKSComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Some static allocations are made outside of what is\n                                             controlled by the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrix           Input coefficient matrix (size: \\c numOccupied × nao) on the GPU. Must not be NULL.\n @param[out] outXCEnergy                 Output (double) for the computed XC energy (size: 1) on the CPU. Must not be NULL.\n                                             This element is overwritten by the calculation results.\n @param[out] outXCPotentialMatrix        Output buffer for computed XC potential matrix (size: nao × nao) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialRKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCPotentialRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCPotentialRKSCompute
        .expect("cuEST function `cuestXCPotentialRKSCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outXCEnergy,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC potential matrix computation.\n @ingroup ExchangeCorrelation\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan and pair list. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestXCPotentialRKSComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize           User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                              2 GB is a good default value for device memory. There are diminishing returns for very\n                                              large values of the variableBufferSize. Some static allocations are made outside of what is\n                                              controlled by the variableBufferSize. Host memory is currently unused.\n                                              This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupied                  Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrix            Input coefficient matrix. May be NULL.\n @param      outXCEnergy                  Output (double) for the computed XC energy. May be NULL.\n @param      outXCPotentialMatrix         Output buffer for computed XC potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialRKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCPotentialRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCPotentialRKSComputeWorkspaceQuery.expect(
        "cuEST function `cuestXCPotentialRKSComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        numOccupied,
        coefficientMatrix,
        outXCEnergy,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Compute the UKS XC potential (Vxc) matrix using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup ExchangeCorrelation\n\n This routine computes the UKS XC potential matrices (Vxc) from occupied molecular orbitals.\n This is the UKS interface that allows the alpha and beta spin orbitals to be different. The user must\n provide occupied molecular orbitals (size nocc x nao) and preallocated output buffers (size nao × nao).\n The orbital basis and integration grid used are passed via the \\c plan. All required temporary workspace\n must be allocated using the size returned by \\c cuestXCPotentialUKSComputeWorkspaceQuery.\n\n Alpha and beta coefficient matrices must always be provided with non-zero numbers of occupied orbitals.\n Cases without beta (or alpha) electrons (hydrogen atom, for example), can be handled by passing in a\n coefficient matrix containing only zeros.\n\n @param[in]  handle                          cuEST handle. Must not be NULL.\n @param[in]  plan                            XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                                 The \\c cuestXCIntPlanParametersFunctional_t set during plan construction determines what XC potential is evaluated.\n @param[in]  parameters                      Compute parameters (\\c cuestXCPotentialUKSComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize              User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                 2 GB is a good default value for device memory. There are diminishing returns for very\n                                                 large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                 controlled by the variableBufferSize. Host memory is currently unused.\n                                                 This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace              Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupiedAlpha                Number of alpha occupied orbitals (rows in alpha coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta                 Number of beta occupied orbitals (rows in beta coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrixAlpha          Input alpha coefficient matrix (size: \\c numOccupiedAlpha × nao). Must not be NULL.\n @param[in]  coefficientMatrixBeta           Input beta coefficient matrix (size: \\c numOccupiedBeta × nao). Must not be NULL.\n @param[out] outXCEnergy                     Output (double) for the computed XC energy (size: 1) on the CPU. Must not be NULL.\n @param[out] outXCPotentialMatrixAlpha       Output buffer for computed XC potential matrix (size: nao × nao) on the GPU. Must not be NULL.\n                                                 Elements of this array are overwritten by the calculation results.\n @param[out] outXCPotentialMatrixBeta        Output buffer for computed XC potential matrix (size: nao × nao) on the GPU. Must not be NULL.\n                                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialUKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCPotentialUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrixAlpha: *mut f64,
    outXCPotentialMatrixBeta: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCPotentialUKSCompute
        .expect("cuEST function `cuestXCPotentialUKSCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupiedAlpha,
        numOccupiedBeta,
        coefficientMatrixAlpha,
        coefficientMatrixBeta,
        outXCEnergy,
        outXCPotentialMatrixAlpha,
        outXCPotentialMatrixBeta,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC potential matrix computation.\n @ingroup ExchangeCorrelation\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan and pair list. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestXCPotentialUKSComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize           User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                              2 GB is a good default value for device memory. There are diminishing returns for very\n                                              large values of the variableBufferSize. Some static allocations are made outside of what is\n                                              controlled by the variableBufferSize. Host memory is currently unused.\n                                              This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupiedAlpha             Number of alpha occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta              Number of beta occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrixAlpha       Input alpha coefficient matrix. May be NULL.\n @param      coefficientMatrixBeta        Input beta coefficient matrix. May be NULL.\n @param      outXCEnergy                  Output (double) for the computed XC energy. May be NULL.\n @param      outXCPotentialMatrixAlpha    Output buffer for computed alpha XC potential matrix. May be NULL.\n @param      outXCPotentialMatrixBeta     Output buffer for computed beta XC potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialUKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCPotentialUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrixAlpha: *mut f64,
    outXCPotentialMatrixBeta: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCPotentialUKSComputeWorkspaceQuery.expect(
        "cuEST function `cuestXCPotentialUKSComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        numOccupiedAlpha,
        numOccupiedBeta,
        coefficientMatrixAlpha,
        coefficientMatrixBeta,
        outXCEnergy,
        outXCPotentialMatrixAlpha,
        outXCPotentialMatrixBeta,
    )
}

#[doc = " @brief Compute the electron density on the integration grid using the provided XC integral plan and molecular orbitals.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine evaluates the electronic density (and its derivatives, when applicable) on the molecular integration grid.\n The computation depends on the chosen approximation level (LDA, GGA, or meta-GGA), which determines the number of\n quantities calculated per grid point. The density is computed from the occupied molecular orbitals.\n\n Each approximation produces an output array of shape (\\c npoint × ndim), where:\n   - LDA: ndim = 1 (\\f$ \\rho \\f$);\n   - GGA: ndim = 4 (\\f$ \\rho, \\nabla_{x}\\rho, \\nabla_{y}\\rho, \\nabla_{z}\\rho \\f$);\n   - meta-GGA: ndim = 5 (\\f$ \\rho, \\nabla_{x}\\rho, \\nabla_{y}\\rho, \\nabla_{z}\\rho, \\tau \\f$).\n\n The returned density does not include integration weights. To obtain the total number of electrons, evaluate:\n \\f[ N_{elec} = \\sum_{P}^{\\rm npoint} w_{P} \\rho_{P} \\f]\n where, \\f$ w_{P} \\f$ is the total integration weight that can be obtained by calling \\c cuestXCIntegrationWeightCompute.\n\n All required workspace must first be allocated according to the size returned by\n \\c cuestXCDensityComputeWorkspaceQuery, which must be called before invoking this function.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation               Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                  Compute parameters (\\c cuestXCDensityComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Some static allocations are made outside of what is\n                                             controlled by the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Preallocated temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in \\c coefficientMatrix). Must be greater than zero.\n @param[in]  coefficientMatrix           Input orbital coefficient matrix (size: \\c numOccupied × nao) on the GPU. Must not be NULL.\n @param[out] outGridDensity              Output buffer for computed density or density derivatives on the grid (size: \\c npoint × ndim) on the GPU. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDensityCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCDensityComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGridDensity: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCDensityCompute
        .expect("cuEST function `cuestXCDensityCompute` not found in the loaded library")(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outGridDensity,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC density computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines the GPU memory requirements for computing the electron density (and its derivatives)\n on the molecular grid using the provided XC integral plan and approximation type. Only memory\n requirements are estimated; no actual computation is performed. Input and output pointers may be NULL.\n\n This call must precede \\c cuestXCDensityCompute to ensure that sufficient temporary memory is available.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation                     Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                        Compute parameters (\\c cuestXCDensityComputeParameters_t) Must not be NULL\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                   controlled by the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor with required workspace sizes (in bytes). Must not be NULL.\n @param[in]  numOccupied                       Number of occupied orbitals (rows in \\c coefficientMatrix). Must be greater than zero.\n @param      coefficientMatrix                 Input orbital coefficient matrix. May be NULL.\n @param      outGridDensity                    Output buffer for computed density values. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDensityComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCDensityComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGridDensity: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCDensityComputeWorkspaceQuery.expect(
        "cuEST function `cuestXCDensityComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        numOccupied,
        coefficientMatrix,
        outGridDensity,
    )
}

#[doc = " @brief Compute the XC potential matrix (Vxc) from a precomputed XC potential on the integration grid.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine constructs the exchange-correlation potential matrix in the AO basis from a grid representation\n of the XC potential. The input grid potential should correspond to the same grid and basis information\n specified by the provided XC integral plan. The dimensionality of the grid potential depends on the selected\n approximation (LDA, GGA, meta-GGA).\n\n Each approximation requires an input array of shape (\\c npoint × ndim) containing the XC potential\n components (derivatives of the XC energy density \\f$ f \\f$) ordered as:\n   - LDA: ndim = 1 (\\f$ \\partial f / \\partial\\rho \\f$);\n   - GGA: ndim = 4 (\\f$ \\partial f / \\partial\\rho,\\; 4 (\\partial f / \\partial\\gamma) \\nabla_{x}\\rho,\\; 4 (\\partial f / \\partial\\gamma) \\nabla_{y}\\rho,\\; 4 (\\partial f / \\partial\\gamma) \\nabla_{z}\\rho \\f$, where \\f$ \\gamma = |\\nabla\\rho|^{2} \\f$);\n   - meta-GGA: ndim = 5 (the GGA components followed by \\f$ \\partial f / \\partial\\tau \\f$).\n\n Important: This function does not apply the integration weights to the user-supplied XC potential. The user must\n apply the total integration weights, which can be obtained by calling \\c cuestXCIntegrationWeightCompute.\n\n All required temporary workspace must first be allocated using the size returned by\n \\c cuestXCPotentialComputeWorkspaceQuery.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation               Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                  Compute parameters (\\c cuestXCPotentialComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Some static allocations are made outside of what is\n                                             controlled by the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Preallocated temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  gridXCPotential             Input XC potential on the grid (size: \\c npoint × ndim) on GPU. Must not be NULL.\n @param[out] outXCPotentialMatrix        Output buffer for computed XC potential matrix (size: nao × nao) on GPU. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCPotentialComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    gridXCPotential: *const f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCPotentialCompute
        .expect("cuEST function `cuestXCPotentialCompute` not found in the loaded library")(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        gridXCPotential,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC potential matrix computation.\n @ingroup ExchangeCorrelationAdvanced\n\n This function returns the GPU workspace memory requirements for constructing\n the exchange-correlation (XC) potential matrix in the AO basis from a given XC potential on the grid.\n Only workspace sizes are computed; the input and output pointers may be NULL.\n\n This query must be executed prior to calling \\c cuestXCPotentialCompute.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation                Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                   Compute parameters (\\c cuestXCPotentialComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize           User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                              2 GB is a good default value for device memory. There are diminishing returns for very\n                                              large values of the variableBufferSize. Some static allocations are made outside of what is\n                                              controlled by the variableBufferSize. Host memory is currently unused.\n                                              This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required temporary workspace sizes (in bytes). Must not be NULL.\n @param      gridXCPotential              Input XC potential on the grid. May be NULL.\n @param      outXCPotentialMatrix         Output buffer for computed XC potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCPotentialComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCPotentialComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    gridXCPotential: *const f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCPotentialComputeWorkspaceQuery.expect(
        "cuEST function `cuestXCPotentialComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        gridXCPotential,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Compute the nonsymmetric electron density on the integration grid using the provided XC integral plan and two sets of molecular orbitals.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine evaluates the nonsymmetric electronic density (and its derivatives, when applicable) on the molecular\n integration grid. The nonsymmetric density is formed from a left and a right set of occupied molecular orbitals,\n both of size \\c numOccupied × nao. This is the generalization of \\c cuestXCDensityCompute, which uses the same\n coefficients for both the left and right.\n\n The output array has the same shape as \\c cuestXCDensityCompute for the chosen approximation (size: \\c npoint × ndim):\n   - LDA: ndim = 1 (\\f$ \\rho \\f$);\n   - GGA: ndim = 4 (\\f$ \\rho, \\nabla_{x}\\rho, \\nabla_{y}\\rho, \\nabla_{z}\\rho \\f$);\n   - meta-GGA: ndim = 5 (\\f$ \\rho, \\nabla_{x}\\rho, \\nabla_{y}\\rho, \\nabla_{z}\\rho, \\tau \\f$).\n\n All required workspace must first be allocated according to the size returned by\n \\c cuestXCNonsymmetricDensityComputeWorkspaceQuery, which must be called before invoking this function.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation               Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                  Compute parameters (\\c cuestXCNonsymmetricDensityComputeParameters_t). Must not be NULL.\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Preallocated temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in each coefficient matrix). Must be greater than zero.\n @param[in]  leftCoefficientMatrix       Input left orbital coefficient matrix (size: \\c numOccupied × nao) on the GPU. Must not be NULL.\n @param[in]  rightCoefficientMatrix      Input right orbital coefficient matrix (size: \\c numOccupied × nao) on the GPU. Must not be NULL.\n @param[out] outGridDensity              Output buffer for computed nonsymmetric density or density derivatives on the grid (size: \\c npoint × ndim) on the GPU. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCNonsymmetricDensityCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCNonsymmetricDensityComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrix: *const f64,
    outGridDensity: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCNonsymmetricDensityCompute.expect(
        "cuEST function `cuestXCNonsymmetricDensityCompute` not found in the loaded library",
    )(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        leftCoefficientMatrix,
        rightCoefficientMatrix,
        outGridDensity,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC nonsymmetric density computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines the GPU memory requirements for computing the nonsymmetric electron density (and its derivatives)\n on the molecular grid using the provided XC integral plan and approximation type. Only memory\n requirements are estimated; no actual computation is performed. Input and output pointers may be NULL.\n\n This call must precede \\c cuestXCNonsymmetricDensityCompute to ensure that sufficient temporary memory is available.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation                     Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                        Compute parameters (\\c cuestXCNonsymmetricDensityComputeParameters_t). Must not be NULL.\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor with required workspace sizes (in bytes). Must not be NULL.\n @param[in]  numOccupied                       Number of occupied orbitals (rows in each coefficient matrix). Must be greater than zero.\n @param      leftCoefficientMatrix             Input left orbital coefficient matrix. May be NULL.\n @param      rightCoefficientMatrix            Input right orbital coefficient matrix. May be NULL.\n @param      outGridDensity                    Output buffer for computed nonsymmetric density values. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are of incorrect type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or \\c approximation is invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCNonsymmetricDensityComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCNonsymmetricDensityComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrix: *const f64,
    outGridDensity: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCNonsymmetricDensityComputeWorkspaceQuery.expect("cuEST function `cuestXCNonsymmetricDensityComputeWorkspaceQuery` not found in the loaded library")(handle, plan, approximation, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, leftCoefficientMatrix, rightCoefficientMatrix, outGridDensity)
}

#[doc = " @brief Compute the derivative of the RKS XC energy (Exc) using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup ExchangeCorrelationGrad\n\n This routine computes the XC gradient from occupied molecular orbitals.\n This is the RKS interface that assumes the alpha and beta spin orbitals are identical. The user must\n provide occupied molecular orbitals (size nocc x nao) and a preallocated output buffer (size natom × 3).\n The orbital basis and integration grid used are passed via the \\c plan. All required temporary workspace\n must be allocated using the size returned by \\c cuestXCDerivativeRKSComputeWorkspaceQuery.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                             The \\c cuestXCIntPlanParametersFunctional_t set during plan construction determines what XC gradient is evaluated.\n @param[in]  parameters                  Compute parameters (\\c cuestXCDerivativeRKSComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Some static allocations are made outside of what is\n                                             controlled by the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrix           Input coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[out] outGradient                 Output buffer for computed XC gradient (size: natom × 3) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDerivativeRKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCDerivativeRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCDerivativeRKSCompute
        .expect("cuEST function `cuestXCDerivativeRKSCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC derivative computation.\n @ingroup ExchangeCorrelationGrad\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestXCDerivativeRKSComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                   controlled by the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupied                       Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrix                 Input coefficient matrix. May be NULL.\n @param      outGradient                       Output buffer for computed XC gradient. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDerivativeRKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCDerivativeRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCDerivativeRKSComputeWorkspaceQuery.expect("cuEST function `cuestXCDerivativeRKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, coefficientMatrix, outGradient)
}

#[doc = " @brief Compute the derivative of the UKS XC energy (Exc) using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup ExchangeCorrelationGrad\n\n This routine computes the UKS XC gradient from occupied molecular orbitals.\n This is the UKS interface that allows the alpha and beta spin orbitals to be different. The user must\n provide occupied molecular orbitals (size nocc x nao) and preallocated output buffer (size natom × 3).\n The orbital basis and integration grid used are passed via the \\c plan. All required temporary workspace\n must be allocated using the size returned by \\c cuestXCDerivativeUKSComputeWorkspaceQuery.\n\n Alpha and beta coefficient matrices must always be provided with non-zero numbers of occupied orbitals.\n Cases without beta (or alpha) electrons (hydrogen atom, for example), can be handled by passing in a\n coefficient matrix containing only zeros.\n\n @param[in]  handle                          cuEST handle. Must not be NULL.\n @param[in]  plan                            XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                                 The \\c cuestXCIntPlanParametersFunctional_t set during plan construction determines what XC gradient is evaluated.\n @param[in]  parameters                      Compute parameters (\\c cuestXCDerivativeUKSComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize              User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                 2 GB is a good default value for device memory. There are diminishing returns for very\n                                                 large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                 controlled by the variableBufferSize. Host memory is currently unused.\n                                                 This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace              Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupiedAlpha                Number of alpha occupied orbitals (rows in alpha coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta                 Number of beta occupied orbitals (rows in beta coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrixAlpha          Input alpha coefficient matrix (size: \\c numOccupiedAlpha × nao) on GPU. Must not be NULL.\n @param[in]  coefficientMatrixBeta           Input beta coefficient matrix (size: \\c numOccupiedBeta × nao) on GPU. Must not be NULL.\n @param[out] outGradient                     Output buffer for computed XC gradient (size: natom × 3) on GPU. Must not be NULL.\n                                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDerivativeUKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCDerivativeUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCDerivativeUKSCompute
        .expect("cuEST function `cuestXCDerivativeUKSCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupiedAlpha,
        numOccupiedBeta,
        coefficientMatrixAlpha,
        coefficientMatrixBeta,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC derivative computation.\n @ingroup ExchangeCorrelationGrad\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                        Compute parameters (\\c cuestXCDerivativeUKSComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                   controlled by the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupiedAlpha                  Number of alpha occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta                   Number of beta occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrixAlpha            Input alpha coefficient matrix. May be NULL.\n @param      coefficientMatrixBeta             Input beta coefficient matrix. May be NULL.\n @param      outGradient                       Output buffer for computed XC gradient. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero;\n   - \\c CUEST_STATUS_EXCEPTION if \\c cuestXCIntPlanParametersFunctional_t is \\c CUEST_XCINTPLAN_PARAMETERS_FUNCTIONAL_HF (set in the \\c cuestXCIntPlan_t);\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDerivativeUKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCDerivativeUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCDerivativeUKSComputeWorkspaceQuery.expect("cuEST function `cuestXCDerivativeUKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupiedAlpha, numOccupiedBeta, coefficientMatrixAlpha, coefficientMatrixBeta, outGradient)
}

#[doc = " @brief Compute the exchange-correlation (XC) derivative contributions for analytical gradients.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine computes the gradient contributions of the exchange–correlation (XC) energy\n with respect to the atomic coordinates. It uses the provided XC potential on the molecular\n integration grid, the molecular orbital coefficients, and the XC integral plan.\n\n The form and dimensionality of the input \\c gridXCPotential depend on the chosen\n approximation type (LDA, GGA, or meta-GGA) - see \\c cuestXCPotentialCompute.\n All temporary workspace must be preallocated\n according to the size returned by \\c cuestXCDerivativeComputeWorkspaceQuery.\n\n Important: This function does not apply the integration weights to the user-supplied XC potential. The user must\n apply the total integration weights, which can be obtained by calling \\c cuestXCIntegrationWeightCompute.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation               Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                  Compute parameters (\\c cuestXCDerivativeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Some static allocations are made outside of what is\n                                             controlled by the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Preallocated temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals in the input coefficient matrix. Must be greater than zero.\n @param[in]  coefficientMatrix           Input orbital coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[in]  gridXCPotential             Input grid exchange-correlation potential array (size: \\c npoint × ndim) on GPU. Must not be NULL.\n @param[out] outBasisGradient            Output buffer for basis function center gradient contributions (size: natom × 3) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n @param[out] outGridGradient             Output buffer for grid point gradient contributions (size: npoint × 3) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or approximation invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n\n In order to compute the derivative of the XC energy,\n \\f[ \\frac{\\rm d}{{\\rm d}R_A} E_{xc} = \\frac{\\rm d}{{\\rm d}R_A} \\sum_{P}^{\\rm npoint} w_{P} e_{xc,P} \\f]\n where \\f$ A \\f$ indexes atom centers and \\f$ P \\f$ are the grid points, there are two terms to evaluate.\n \\f[ \\frac{\\rm d}{{\\rm d}R_A} E_{xc} = \\sum_{P}^{\\rm npoint} \\frac{{\\rm d} w_{P}}{{\\rm d}R_A} e_{xc,P} + \\sum_{P}^{\\rm npoint} w_{P} \\frac{{\\rm d} e_{xc,P}}{{\\rm d}R_A} \\f]\n Contributions from the second term, \\f$ \\sum_{P}^{\\rm npoint} w_{P} \\frac{{\\rm d} e_{xc,P}}{{\\rm d}R_A} \\f$ are evaluated by this function. Specifically,\n this function computes \\c outBasisGradient computed as \\f$ \\sum_{P}^{\\rm npoint} w_{P} \\frac{\\partial e_{xc,P}}{\\partial R_A} \\f$, where \\f$ R_A \\f$ are the basis\n function centers (treated as separate from grid point centers). It also computes \\c outGridGradient as \\f$ \\sum_{P}^{\\rm npoint} w_{P} \\frac{\\partial e_{xc,P}}{\\partial R_Q} \\f$,\n where \\f$ R_Q \\f$ are the grid points. The \\c outBasisGradient contributed directly to the derivative of the XC energy. The \\c outGridGradient array should be included\n in a subsequent call to \\c cuestXCGridDerivativeCompute to assemble additional contributions to the gradient.\n"]
pub unsafe fn cuestXCDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    gridXCPotential: *const f64,
    outBasisGradient: *mut f64,
    outGridGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCDerivativeCompute
        .expect("cuEST function `cuestXCDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        gridXCPotential,
        outBasisGradient,
        outGridGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC derivative computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines the required GPU workspace size to compute exchange–correlation (XC) energy gradient\n contributions. This query prepares for \\c cuestXCDerivativeCompute by reporting the necessary\n temporary memory allocation given the chosen approximation and molecular system.\n\n Only workspace sizes are computed; inputs/outputs may be NULL.\n The same \\c variableBufferSize descriptor must be reused for the corresponding compute call.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation                     Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                        Compute parameters (\\c cuestXCDerivativeComputeParameters_t). Must not be NULL\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Some static allocations are made outside of what is\n                                                   controlled by the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor with required workspace sizes (in bytes). Must not be NULL.\n @param[in]  numOccupied                       Number of occupied orbitals. Must be greater than zero.\n @param      coefficientMatrix                 Input coefficient matrix. May be NULL.\n @param      gridXCPotential                   Input grid XC potential array. May be NULL.\n @param      outBasisGradient                  Output buffer for basis function center gradient contributions. May be NULL.\n @param      outGridGradient                   Output buffer for grid point gradient contributions. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or approximation invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    gridXCPotential: *const f64,
    outBasisGradient: *mut f64,
    outGridGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCDerivativeComputeWorkspaceQuery.expect(
        "cuEST function `cuestXCDerivativeComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspaceDescriptor,
        numOccupied,
        coefficientMatrix,
        gridXCPotential,
        outBasisGradient,
        outGridGradient,
    )
}

#[doc = " @brief Compute total XC energy gradient from grid XC energy and its spatial derivatives.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine combines grid-resolved XC energy densities and their spatial gradients\n to produce the total (molecular) XC gradient with respect to atomic coordinates.\n It serves as the final step in assembling analytical derivatives after the\n grid-level contributions have been computed.\n\n The operation uses the molecular grid and AO basis data embedded in the provided XC plan.\n All required workspace should be preallocated according to the size returned by\n \\c cuestXCGridDerivativeComputeWorkspaceQuery.\n\n Important: This function does apply integration weights to the user-supplied XC energy. The user must\n supply the bare \\f$ e_{xc,P} \\f$ array as shown in:\n \\f[ E_{xc} = \\sum_{P}^{\\rm npoint} w_{P} e_{xc,P} \\f]\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestXCGridDerivativeComputeParameters_t). Must not be NULL\n @param[in]  temporaryWorkspace Preallocated GPU workspace buffer for temporary data. Must not be NULL.\n @param[in]  gridXCEnergy       Input grid XC energy density values (size: npoint × 1) on GPU. Must not be NULL.\n @param[in]  gridGradient       Input grid spatial gradient contributions (size: npoint × 3) on GPU. Must not be NULL.\n @param[out] outGradient        Output total (atomic) gradient of XC energy (size: natom × 3) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise.\n\n This function is expected to be called after \\c cuestXCDerivativeCompute. The purpose of this function\n is to evaluate contributions to the XC gradient resulting from derivatives of the integration weight\n \\f$ \\sum_{P}^{\\rm npoint} \\frac{{\\rm d} w_{P}}{{\\rm d}R_A} e_{xc,P} \\f$ and to incorporate contributions\n from grid point motion: \\f$ \\sum_{P}^{\\rm npoint} w_{P} \\frac{\\partial e_{xc,P}}{\\partial R_Q} \\frac{\\partial R_Q}{\\partial R_A} \\f$.\n Here \\f$ R_Q \\f$ are the grid points and \\f$ R_A \\f$ are the atomic centers defining the grid.\n\n The sum of \\c outGradient computed by this routine and \\c outBasisGradient computed by \\c cuestXCDerivativeCompute (scaled appropriately)\n is the derivative of the XC energy: \\f$ \\frac{\\rm d}{{\\rm d}R_A} E_{xc} \\f$. In the case of UKS calculations, \\c cuestXCDerivativeCompute\n will need to be called twice, but \\c cuestXCGridDerivativeCompute only needs to be called once."]
pub unsafe fn cuestXCGridDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCGridDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    gridXCEnergy: *const f64,
    gridGradient: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestXCGridDerivativeCompute
        .expect("cuEST function `cuestXCGridDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        gridXCEnergy,
        gridGradient,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC grid derivative computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Estimates the GPU memory workspace required to compute the grid-based exchange–correlation (XC)\n gradient using molecular grid data. This function reports the necessary device buffer size\n for \\c cuestXCGridDerivativeCompute.\n\n Only workspace sizes are computed; input and output buffers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestXCGridDerivativeComputeParameters_t). Must not be NULL\n @param[out] temporaryWorkspaceDescriptor Output descriptor with required temporary workspace sizes (in bytes). Must not be NULL.\n @param      gridXCEnergy                 Input grid XC energy density. May be NULL.\n @param      gridGradient                 Input grid gradient contributions. May be NULL.\n @param      outGradient                  Output total (atomic) gradient array. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCGridDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestXCGridDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    gridXCEnergy: *const f64,
    gridGradient: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCGridDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestXCGridDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, gridXCEnergy, gridGradient, outGradient)
}

#[doc = " @brief Compute the nonsymmetric exchange-correlation (XC) derivative contributions for analytical gradients.\n @ingroup ExchangeCorrelationAdvanced\n\n This routine computes the gradient contributions of the exchange–correlation (XC) energy\n with respect to the atomic coordinates using a nonsymmetric density. It uses the provided XC\n potential on the molecular integration grid, a left and a right set of molecular orbital\n coefficients, and the XC integral plan. This is the generalization of \\c cuestXCDerivativeCompute,\n which uses the same coefficients for both left and right.\n\n The form and dimensionality of the input \\c gridXCPotential depend on the chosen\n approximation type (LDA, GGA, or meta-GGA) — see \\c cuestXCPotentialCompute.\n All temporary workspace must be preallocated according to the size returned by\n \\c cuestXCNonsymmetricDerivativeComputeWorkspaceQuery.\n\n Important: This function does not apply the integration weights to the user-supplied XC potential. The user must\n apply the total integration weights, which can be obtained by calling \\c cuestXCIntegrationWeightCompute.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation               Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                  Compute parameters (\\c cuestXCNonsymmetricDerivativeComputeParameters_t). Must not be NULL.\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Preallocated temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals in each coefficient matrix. Must be greater than zero.\n @param[in]  leftCoefficientMatrix       Input left orbital coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[in]  rightCoefficientMatrix      Input right orbital coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[in]  gridXCPotential             Input grid exchange-correlation potential array (size: \\c npoint × ndim) on GPU. Must not be NULL.\n @param[out] outBasisGradient            Output buffer for basis function center gradient contributions (size: natom × 3) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n @param[out] outGridGradient             Output buffer for grid point gradient contributions (size: npoint × 3) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or approximation invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCNonsymmetricDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCNonsymmetricDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrix: *const f64,
    gridXCPotential: *const f64,
    outBasisGradient: *mut f64,
    outGridGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCNonsymmetricDerivativeCompute.expect(
        "cuEST function `cuestXCNonsymmetricDerivativeCompute` not found in the loaded library",
    )(
        handle,
        plan,
        approximation,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        leftCoefficientMatrix,
        rightCoefficientMatrix,
        gridXCPotential,
        outBasisGradient,
        outGridGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for an XC nonsymmetric derivative computation.\n @ingroup ExchangeCorrelationAdvanced\n\n Determines the required GPU workspace size to compute nonsymmetric exchange–correlation (XC) energy gradient\n contributions. This query prepares for \\c cuestXCNonsymmetricDerivativeCompute by reporting the necessary\n temporary memory allocation given the chosen approximation and molecular system.\n\n Only workspace sizes are computed; inputs/outputs may be NULL.\n The same \\c variableBufferSize descriptor must be reused for the corresponding compute call.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  approximation                     Specifies the XC approximation: LDA, GGA, or meta-GGA.\n @param[in]  parameters                        Compute parameters (\\c cuestXCNonsymmetricDerivativeComputeParameters_t). Must not be NULL.\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor with required workspace sizes (in bytes). Must not be NULL.\n @param[in]  numOccupied                       Number of occupied orbitals. Must be greater than zero.\n @param      leftCoefficientMatrix             Input left coefficient matrix. May be NULL.\n @param      rightCoefficientMatrix            Input right coefficient matrix. May be NULL.\n @param      gridXCPotential                   Input grid XC potential array. May be NULL.\n @param      outBasisGradient                  Output buffer for basis function center gradient contributions. May be NULL.\n @param      outGridGradient                   Output buffer for grid point gradient contributions. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if handle types are incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero or approximation invalid;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestXCNonsymmetricDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    approximation: cuestXCAdvancedComputeParametersApproximation_t,
    parameters: cuestXCNonsymmetricDerivativeComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    leftCoefficientMatrix: *const f64,
    rightCoefficientMatrix: *const f64,
    gridXCPotential: *const f64,
    outBasisGradient: *mut f64,
    outGridGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestXCNonsymmetricDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestXCNonsymmetricDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, approximation, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, leftCoefficientMatrix, rightCoefficientMatrix, gridXCPotential, outBasisGradient, outGridGradient)
}

#[doc = " @brief Compute the nonlocal RKS XC potential (Vxc) matrix using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup NonLocalCorrelation\n\n This routine computes the XC potential matrix (Vxc) from the occupied molecular orbitals.\n This is the RKS interface that assumes the alpha and beta spin densities are identical. The user must\n provide preallocated occupied molecular orbitals (size: nocc x nao)\n and a preallocated output buffer (size: nao × nao). The orbital basis and integration grid used are passed\n via the \\c plan . All required temporary workspace must be allocated using the size returned by\n \\c cuestNonlocalXCPotentialRKSComputeWorkspaceQuery.\n\n \\b Precision selection. This routine can use reduced-precision \"ffloat\" compute, selected through the\n compute parameters:\n   - \\c CUEST_NONLOCALXCPOTENTIALRKSCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t):\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                         The \\c cuestXCIntPlanParametersFunctional_t set during plan construction does not affect this compute routine.\n @param[in]  parameters                  \\c cuestNonlocalXCPotentialRKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize          User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                             2 GB is a good default value for device memory. There are diminishing returns for very\n                                             large values of the variableBufferSize. Host memory is currently unused.\n                                             This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                 Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrix           Input coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[out] outXCEnergy                 Output (double) for the computed XC energy (size: 1) on CPU. Must not be NULL.\n @param[out] outXCPotentialMatrix        Output buffer for computed XC potential matrix (size: nao × nao) on GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCPotentialRKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCPotentialRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCPotentialRKSCompute.expect(
        "cuEST function `cuestNonlocalXCPotentialRKSCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outXCEnergy,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a nonlocal XC potential matrix computation.\n @ingroup NonLocalCorrelation\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan and pair list. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   \\c cuestNonlocalXCPotentialRKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize           User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                              2 GB is a good default value for device memory. There are diminishing returns for very\n                                              large values of the variableBufferSize. Host memory is currently unused.\n                                              This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupied                  Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrix            Input coefficient matrix. May be NULL.\n @param      outXCEnergy                  Output (double) for the computed XC energy. May be NULL.\n @param      outXCPotentialMatrix         Output buffer for computed XC potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCPotentialRKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCPotentialRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCPotentialRKSComputeWorkspaceQuery.expect("cuEST function `cuestNonlocalXCPotentialRKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, coefficientMatrix, outXCEnergy, outXCPotentialMatrix)
}

#[doc = " @brief Compute the nonlocal UKS XC potential (Vxc) matrix using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup NonLocalCorrelation\n\n This routine computes the UKS XC potential matrix (Vxc) from occupied molecular orbitals.\n This is the UKS interface that allows the alpha and beta spin densities to be different. The user must\n provide preallocated occupied molecular orbitals (size: noccA x nao and noccB x nao)\n and preallocated output buffer (size: nao × nao). The orbital basis and integration grid used are passed\n via the \\c plan. All required temporary workspace must be allocated using the size returned by\n \\c cuestNonlocalXCPotentialUKSComputeWorkspaceQuery.\n\n Alpha and beta coefficient matrices must always be provided with non-zero numbers of occupied orbitals.\n Cases without beta (or alpha) electrons (hydrogen atom, for example), can be handled by passing in a\n coefficient matrix containing only zeros.\n\n \\b Precision selection. This routine can use reduced-precision \"ffloat\" compute, selected through the\n compute parameters:\n   - \\c CUEST_NONLOCALXCPOTENTIALUKSCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t):\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n\n @param[in]  handle                     cuEST handle. Must not be NULL.\n @param[in]  plan                       XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                            The \\c cuestXCIntPlanParametersFunctional_t set during plan construction does not affect this compute routine.\n @param[in]  parameters                 \\c cuestNonlocalXCPotentialUKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize         User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                            2 GB is a good default value for device memory. There are diminishing returns for very\n                                            large values of the variableBufferSize. Host memory is currently unused.\n                                            This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace         Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupiedAlpha           Number of alpha occupied orbitals (rows in alpha coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta            Number of beta occupied orbitals (rows in beta coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrixAlpha     Input alpha coefficient matrix (size: \\c numOccupiedAlpha × nao) on GPU. Must not be NULL.\n @param[in]  coefficientMatrixBeta      Input beta coefficient matrix (size: \\c numOccupiedBeta × nao) on GPU. Must not be NULL.\n @param[out] outXCEnergy                Output (double) for the computed XC energy (size: 1) on CPU. Must not be NULL.\n @param[out] outXCPotentialMatrix       Output buffer for computed XC potential matrix (size: nao × nao) on GPU. Must not be NULL.\n                                        Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCPotentialUKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCPotentialUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCPotentialUKSCompute.expect(
        "cuEST function `cuestNonlocalXCPotentialUKSCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupiedAlpha,
        numOccupiedBeta,
        coefficientMatrixAlpha,
        coefficientMatrixBeta,
        outXCEnergy,
        outXCPotentialMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a nonlocal XC potential matrix computation.\n @ingroup NonLocalCorrelation\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan and pair list. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   \\c cuestNonlocalXCPotentialUKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize           User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                              2 GB is a good default value for device memory. There are diminishing returns for very\n                                              large values of the variableBufferSize. Host memory is currently unused.\n                                              This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupiedAlpha             Number of alpha occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta              Number of beta occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrixAlpha       Input alpha coefficient matrix. May be NULL.\n @param      coefficientMatrixBeta        Input beta coefficient matrix. May be NULL.\n @param      outXCEnergy                  Output (double) for the computed XC energy. May be NULL.\n @param      outXCPotentialMatrix         Output buffer for computed XC potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCPotentialUKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCPotentialUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outXCEnergy: *mut f64,
    outXCPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCPotentialUKSComputeWorkspaceQuery.expect("cuEST function `cuestNonlocalXCPotentialUKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupiedAlpha, numOccupiedBeta, coefficientMatrixAlpha, coefficientMatrixBeta, outXCEnergy, outXCPotentialMatrix)
}

#[doc = " @brief Compute the derivative of the nonlocal RKS XC energy (Exc) using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup NonLocalCorrelationGrad\n\n This routine computes the XC gradient from the occupied molecular orbitals.\n This is the RKS interface that assumes the alpha and beta spin densities are identical. The user must\n provide preallocated occupied molecular orbitals (size nocc x nao)\n and a preallocated output buffer (size natom × 3). The orbital basis and integration grid used are passed\n via the \\c plan . All required temporary workspace must be allocated using the size returned by\n \\c cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery.\n\n \\b Precision selection. This routine can use reduced-precision \"ffloat\" compute, selected through the\n compute parameters:\n   - \\c CUEST_NONLOCALXCDERIVATIVERKSCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t):\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n\n @param[in]  handle                     cuEST handle. Must not be NULL.\n @param[in]  plan                       XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                            The \\c cuestXCIntPlanParametersFunctional_t set during plan construction does not affect this compute routine.\n @param[in]  parameters                 \\c cuestNonlocalXCDerivativeRKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize         User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                            2 GB is a good default value for device memory. There are diminishing returns for very\n                                            large values of the variableBufferSize. Host memory is currently unused.\n                                            This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace         Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupied                Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrix          Input coefficient matrix (size: \\c numOccupied × nao) on GPU. Must not be NULL.\n @param[out] outGradient                Output buffer for computed XC gradient (size: natom × 3) on GPU. Must not be NULL.\n                                            Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCDerivativeRKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCDerivativeRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCDerivativeRKSCompute.expect(
        "cuEST function `cuestNonlocalXCDerivativeRKSCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupied,
        coefficientMatrix,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a nonlocal XC derivative computation.\n @ingroup NonLocalCorrelationGrad\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                         cuEST handle. Must not be NULL.\n @param[in]  plan                           XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                     \\c cuestNonlocalXCDerivativeRKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize             User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                2 GB is a good default value for device memory. There are diminishing returns for very\n                                                large values of the variableBufferSize. Host memory is currently unused.\n                                                This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor   Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupied                    Number of occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrix              Input coefficient matrix. May be NULL.\n @param      outGradient                    Output buffer for computed XC gradient. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupied is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCDerivativeRKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupied: u64,
    coefficientMatrix: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery.expect("cuEST function `cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupied, coefficientMatrix, outGradient)
}

#[doc = " @brief Compute the derivative of the nonlocal UKS XC energy (Exc) using the supplied XC integral plan and occupied molecular orbitals.\n @ingroup NonLocalCorrelationGrad\n\n This routine computes the UKS nonlocal XC gradient from occupied molecular orbitals.\n This is the UKS interface that allows the alpha and beta spin densities to be different. The user must\n provide preallocated occupied molecular orbitals (size noccA x nao and noccB x nao)\n and preallocated output buffer (size natom × 3). The orbital basis and integration grid used are passed\n via the \\c plan . All required temporary workspace must be allocated using the size returned by\n \\c cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery.\n\n Alpha and beta coefficient matrices must always be provided with non-zero numbers of occupied orbitals.\n Cases without beta (or alpha) electrons (hydrogen atom, for example), can be handled by passing in a\n coefficient matrix containing only zeros.\n\n \\b Precision selection. This routine can use reduced-precision \"ffloat\" compute, selected through the\n compute parameters:\n   - \\c CUEST_NONLOCALXCDERIVATIVEUKSCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t):\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n\n @param[in]  handle                          cuEST handle. Must not be NULL.\n @param[in]  plan                            XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n                                                 The \\c cuestXCIntPlanParametersFunctional_t set during plan construction does not affect this compute routine.\n @param[in]  parameters                      \\c cuestNonlocalXCDerivativeUKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize              User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                 2 GB is a good default value for device memory. There are diminishing returns for very\n                                                 large values of the variableBufferSize. Host memory is currently unused.\n                                                 This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[in]  temporaryWorkspace              Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  numOccupiedAlpha                Number of alpha occupied orbitals (rows in alpha coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta                 Number of beta occupied orbitals (rows in beta coefficient matrix); must be greater than zero.\n @param[in]  coefficientMatrixAlpha          Input alpha coefficient matrix (size: \\c numOccupiedAlpha × nao) on GPU. Must not be NULL.\n @param[in]  coefficientMatrixBeta           Input beta coefficient matrix (size: \\c numOccupiedBeta × nao) on GPU. Must not be NULL.\n @param[out] outGradient                     Output buffer for computed XC gradient (size: natom × 3) on GPU. Must not be NULL.\n                                                 Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCDerivativeUKSCompute(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCDerivativeUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCDerivativeUKSCompute.expect(
        "cuEST function `cuestNonlocalXCDerivativeUKSCompute` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        variableBufferSize,
        temporaryWorkspace,
        numOccupiedAlpha,
        numOccupiedBeta,
        coefficientMatrixAlpha,
        coefficientMatrixBeta,
        outGradient,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for a nonlocal XC derivative computation.\n @ingroup NonLocalCorrelationGrad\n\n This function returns the workspace needed (host/device) for a single computation,\n based on the provided plan. Only sizes are computed; output and orbital\n buffers may be NULL.\n\n @param[in]  handle                            cuEST handle. Must not be NULL.\n @param[in]  plan                              XC integral plan (opaque handle) constructed with \\c cuestXCIntPlanCreate. Must not be NULL.\n @param[in]  parameters                        \\c cuestNonlocalXCDerivativeUKSComputeParameters_t defines the details of the nonlocal XC functional.\n @param[in]  variableBufferSize                User-provided descriptor for the variable buffer size (host and device). Must not be NULL.\n                                                   2 GB is a good default value for device memory. There are diminishing returns for very\n                                                   large values of the variableBufferSize. Host memory is currently unused.\n                                                   This value MUST remain unchanged between the WorkspaceQuery call and the Compute call.\n @param[out] temporaryWorkspaceDescriptor      Output descriptor for temporary workspace sizes. Must not be NULL.\n @param[in]  numOccupiedAlpha                  Number of alpha occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param[in]  numOccupiedBeta                   Number of beta occupied orbitals (rows in coefficient matrix); must be greater than zero.\n @param      coefficientMatrixAlpha            Input alpha coefficient matrix (size: \\c numOccupiedAlpha × nao) on GPU. May be NULL.\n @param      coefficientMatrixBeta             Input beta coefficient matrix (size: \\c numOccupiedBeta × nao) on GPU. May be NULL.\n @param      outGradient                       Output buffer for computed XC gradient (size: natom × 3) on GPU. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c numOccupiedAlpha or \\c numOccupiedBeta is zero, or if a parameter value is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestXCIntPlan_t,
    parameters: cuestNonlocalXCDerivativeUKSComputeParameters_t,
    variableBufferSize: *const cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numOccupiedAlpha: u64,
    numOccupiedBeta: u64,
    coefficientMatrixAlpha: *const f64,
    coefficientMatrixBeta: *const f64,
    outGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery.expect("cuEST function `cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, variableBufferSize, temporaryWorkspaceDescriptor, numOccupiedAlpha, numOccupiedBeta, coefficientMatrixAlpha, coefficientMatrixBeta, outGradient)
}

#[doc = " @brief Build a PCM integral plan.\n @ingroup PCMIntegralPlan\n\n All object handles, workspaces, and output pointers must be valid (not NULL).\n Workspace buffers must be preallocated with required sizes (see workspace query).\n\n Note that for common applications of PCM the effectiveNuclearCharges are expected to be positive integers.\n\n @param[in] handle                   cuEST handle. Must not be NULL.\n @param[in] intPlan                  One-electron integral plan handle of type \\c cuestOEIntPlan_t .\n                                         The one-electron integral plan should be created with \\c cuestOEIntPlanCreate and\n                                         must not be destroyed until after the final call to any PCM compute routines. Must not be NULL.\n @param[in] parameters               PCM integral plan parameters structure. Must not be NULL.\n @param[in] persistentWorkspace      Preallocated workspace for persistent buffers. Must not be NULL.\n @param[in] temporaryWorkspace       Preallocated workspace for temporary buffers. Must not be NULL.\n @param[in] numAngularPointsPerAtom  CPU array (size: number of atoms) of number of angular points per atom. Must not be NULL.\n @param[in] epsilon                  Dielectric constant. Must be strictly greater than 1.0.\n @param[in] zetas                    CPU array (size: number of atoms) with each element containing the optimized zeta\n                                         coefficient for the number of angular points placed on that atom.  Recommended values can be found in the API User Guide. All values must be positive.\n @param[in]  atomicRadii             CPU array (size: number of atoms) of atomic radii in bohr; all values must be positive. Must not be NULL.\n @param[in]  effectiveNuclearCharges CPU array (size: number of atoms) of nuclear charges, adjusted for the presence of effective core potentials. Must not be NULL.\n @param[out] outPlan                 Output pointer for PCM plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c epsilon is less than or equal to 1.0, or if any of the \\c zetas or \\c atomicRadii values are invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntPlanCreate(
    handle: cuestHandle_t,
    intPlan: cuestOEIntPlan_t,
    parameters: cuestPCMIntPlanParameters_t,
    persistentWorkspace: *mut cuestWorkspace_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    numAngularPointsPerAtom: *const u64,
    epsilon: f64,
    zetas: *const f64,
    atomicRadii: *const f64,
    effectiveNuclearCharges: *const f64,
    outPlan: *mut cuestPCMIntPlan_t,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMIntPlanCreate
        .expect("cuEST function `cuestPCMIntPlanCreate` not found in the loaded library")(
        handle,
        intPlan,
        parameters,
        persistentWorkspace,
        temporaryWorkspace,
        numAngularPointsPerAtom,
        epsilon,
        zetas,
        atomicRadii,
        effectiveNuclearCharges,
        outPlan,
    )
}

#[doc = " @brief Query workspace requirements for creating a PCM integral plan.\n @ingroup PCMIntegralPlan\n\n Determines persistent and temporary buffer sizes by simulating allocation logic.\n All handles and pointers must be valid (not NULL).\n\n @param[in]  handle                        cuEST handle. Must not be NULL.\n @param[in]  intPlan                       One-electron integral plan handle of type \\c cuestOEIntPlan_t .\n                                               The one-electron integral plan should be created with \\c cuestOEIntPlanCreate and\n                                               must not be destroyed until after the final call to any PCM compute routines.  Must not be NULL.\n @param[in]  parameters                    PCM integral plan parameters structure. Must not be NULL.\n @param[out] persistentWorkspaceDescriptor Descriptor for persistent workspace requirements. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor  Descriptor for temporary workspace requirements. Must not be NULL.\n @param[in]  numAngularPointsPerAtom       CPU array (size: number of atoms) of number of angular points per atom. Must not be NULL.\n @param[in]  epsilon                       Dielectric constant. Must be strictly greater than 1.0.\n @param      zetas                         CPU array with each element containing the optimized zeta coefficient for the number of angular points placed on that atom. May be NULL.\n @param      atomicRadii                   CPU array of atomic radii in bohr. May be NULL.\n @param      effectiveNuclearCharges       CPU array of nuclear charges, adjusted for the presence of effective core potentials. May be NULL.\n @param      outPlan                       Output pointer for PCM plan handle. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer argument is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c epsilon is less than or equal to 1.0;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntPlanCreateWorkspaceQuery(
    handle: cuestHandle_t,
    intPlan: cuestOEIntPlan_t,
    parameters: cuestPCMIntPlanParameters_t,
    persistentWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    numAngularPointsPerAtom: *const u64,
    epsilon: f64,
    zetas: *const f64,
    atomicRadii: *const f64,
    effectiveNuclearCharges: *const f64,
    outPlan: *mut cuestPCMIntPlan_t,
) -> cuestStatus_t {
    dyload_lib().cuestPCMIntPlanCreateWorkspaceQuery.expect(
        "cuEST function `cuestPCMIntPlanCreateWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        intPlan,
        parameters,
        persistentWorkspaceDescriptor,
        temporaryWorkspaceDescriptor,
        numAngularPointsPerAtom,
        epsilon,
        zetas,
        atomicRadii,
        effectiveNuclearCharges,
        outPlan,
    )
}

#[doc = " @brief Destroy a PCM integral plan and release its resources.\n @ingroup PCMIntegralPlan\n\n Destroys the PCM integral plan and releases the resources it holds.\n All handles and pointers must be valid (not NULL).\n\n @param[in] plan                  PCM integral plan handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if the PCM integral plan handle is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the PCM integral plan handle is not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntPlanDestroy(plan: cuestPCMIntPlan_t) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMIntPlanDestroy
        .expect("cuEST function `cuestPCMIntPlanDestroy` not found in the loaded library")(plan)
}

#[doc = " @brief Compute the PCM potential matrix using preconditioned conjugate gradient (PCG) to solve for the PCM charges.\n @ingroup PCM\n\n Guesses are obtained from the \\c inQ matrix, with the pre-allocated \\c outQ matrix used to return the converged charges.\n\n Convergence properties are controlled by configuring the \\c cuestPCMPotentialComputeParameters_t structure passed as the \\c parameters parameter.\n\n The required temporary workspace must be allocated using the sizes returned by \\c cuestPCMPotentialComputeWorkspaceQuery.\n The number of points, npoint, can be queried from the \\c cuestPCMIntPlan_t structure using the \\c CUEST_PCMINTPLAN_NUM_POINT query\n and is the sum of the \\c numAngularPointsPerAtom array used to create the plan.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMPOTENTIALCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMPOTENTIALCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                  PCM potential compute parameters (opaque handle) constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  densityMatrix               Total (alpha + beta) AO basis density matrix (size: nao × nao) on the GPU. Must not be NULL.\n @param[in]  inQ                         Input guess Q vector (size: npoint) on the GPU; user should initialize this array to zeros if no guess is available. Must not be NULL.\n @param[out] outQ                        Output Q matrix (size: npoint) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n @param[out] outPCMResults               PCM results (opaque handle) constructed with \\c cuestResultsCreate. Must not be NULL.\n                                             Elements contained in this handle are overwritten by the calculation results.\n @param[out] outPCMPotentialMatrix       Output PCM potential matrix (size: nao × nao) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n Note that the return status is \\c CUEST_STATUS_SUCCESS even if the PCM charges did not converge within the maximum number of iterations provided by the \\c parameters.\n The caller should check the \\c converged member of the \\c outPCMResults handle to determine if the PCM charges converged.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success, including the case where the PCM charges did not converge within the maximum number of iterations provided by the \\c parameters;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMPotentialCompute(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMPotentialComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMPotentialCompute
        .expect("cuEST function `cuestPCMPotentialCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        inQ,
        outQ,
        outPCMResults,
        outPCMPotentialMatrix,
    )
}

#[doc = " @brief Query the temporary workspace required for a PCM potential computation.\n @ingroup PCM\n\n This function calculates the memory required for a PCM potential computation,\n returning host/device workspace buffer sizes in the output descriptor. The user arrays \\c densityMatrix, \\c inQ, \\c outQ, \\c outPCMResults, and \\c outPCMPotentialMatrix may be NULL.\n All parameters and object handles must be valid.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMPOTENTIALCOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMPOTENTIALCOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                        cuEST handle. Must not be NULL.\n @param[in]  plan                          PCM integral plan constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                    PCM potential compute parameters constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor  Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      densityMatrix                 Density matrix. May be NULL.\n @param      inQ                           Input Q matrix. May be NULL.\n @param      outQ                          Output Q matrix. May be NULL.\n @param      outPCMResults                 PCM results constructed with \\c cuestResultsCreate. May be NULL.\n @param      outPCMPotentialMatrix         Output PCM potential matrix. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMPotentialComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMPotentialComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMPotentialMatrix: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPCMPotentialComputeWorkspaceQuery.expect(
        "cuEST function `cuestPCMPotentialComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        temporaryWorkspaceDescriptor,
        densityMatrix,
        inQ,
        outQ,
        outPCMResults,
        outPCMPotentialMatrix,
    )
}

#[doc = " @brief Compute the PCM nuclear gradient.\n @ingroup PCM\n\n This routine computes the derivative of the PCM energy with respect to nuclear coordinates.\n\n Converged surface charges are computed using a preconditioned conjugate gradient (PCG) solver.\n Convergence behavior is controlled by the \\c cuestPCMDerivativeComputeParameters_t structure that is packaged inside \\c parameters.\n Both maximum iterations and convergence threshold may be tuned based on the specific problem.\n\n The required temporary workspace must be allocated using the sizes returned by \\c cuestPCMDerivativeComputeWorkspaceQuery.\n The number of points, npoint, can be queried from the \\c cuestPCMIntPlan_t structure using the \\c CUEST_PCMINTPLAN_NUM_POINT query,\n and is the sum of the \\c numAngularPointsPerAtom array used to create the plan.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                  PCM derivative parameters (opaque handle) constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  densityMatrix               Total (alpha + beta) AO basis density matrix (size: nao × nao) on the GPU. Must not be NULL.\n @param[in]  inQ                         Input guess charges (size: npoint) on the GPU; user should initialize this array to zeros if no guess is available. Must not be NULL.\n @param[out] outQ                        Converged charges (size: npoint) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n @param[out] outPCMResults               PCM results (opaque handle) constructed with \\c cuestResultsCreate. Must not be NULL.\n                                             Elements contained in this handle are overwritten by the calculation results.\n @param[out] outPCMGradient              Output PCM derivative (size: natoms × 3) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n It is important to note that the return status is \\c CUEST_STATUS_SUCCESS even if the PCM charges did not converge within the maximum number of iterations provided by \\c parameters.\n The caller should check the \\c converged member of the \\c outPCMResults handle to determine whether the PCM charges returned by \\c outQ are converged.\n\n In the case where \\c maxIterations==0 , no PCG updates of the charges are performed.\n The values provided in \\c inQ are used to evaluate the PCM gradient, and are copied to \\c outQ on return.\n In this mode, a residual is still evaluated and reported through \\c outPCMResults, but the charges themselves are not modified.\n\n Similarly, if the PCG procedure reaches the maximum number of iterations without satisfying the requested\n convergence threshold, the gradient is still computed using the resulting (partially converged) charges.\n It is the caller’s responsibility to inspect the convergence information in \\c outPCMResults and decide whether the result is acceptable.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success, including the case where the PCM charges did not converge within the maximum number of iterations provided by the \\c parameters;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type; including \\c parameters;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMDerivativeCompute
        .expect("cuEST function `cuestPCMDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        inQ,
        outQ,
        outPCMResults,
        outPCMGradient,
    )
}

#[doc = " @brief Query the temporary workspace required for a PCM gradient computation.\n @ingroup PCM\n\n Calculate the memory required for a PCM gradient computation, including the PCG solve for charges,\n returning host/device workspace buffer sizes in the output descriptor. The user arrays \\c densityMatrix, \\c inQ,\n \\c outQ, \\c outPCMResults, and \\c outPCMGradient may be NULL. All parameters and object handles must be valid.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                        cuEST handle. Must not be NULL.\n @param[in]  plan                          PCM integral plan constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                    PCM derivative parameters constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor  Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      densityMatrix                 Density matrix. May be NULL.\n @param      inQ                           Input Q vector. May be NULL.\n @param      outQ                          Output Q vector. May be NULL.\n @param      outPCMResults                 PCM results constructed with \\c cuestResultsCreate. May be NULL.\n @param      outPCMGradient                Output PCM nuclear gradient. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type; including \\c parameters;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPCMDerivativeComputeWorkspaceQuery.expect(
        "cuEST function `cuestPCMDerivativeComputeWorkspaceQuery` not found in the loaded library",
    )(
        handle,
        plan,
        parameters,
        temporaryWorkspaceDescriptor,
        densityMatrix,
        inQ,
        outQ,
        outPCMResults,
        outPCMGradient,
    )
}

#[doc = " @brief Compute the PCM gradient with respect to the atomic (cavity) radii.\n @ingroup PCM\n\n This routine computes the derivative of the PCM energy with respect to each atomic cavity radius\n (the quantity needed by radius-rescaling solvent models such as DRACO), holding the electron density fixed.\n\n Converged surface charges are computed using a preconditioned conjugate gradient (PCG) solver.\n Convergence behavior is controlled by the \\c cuestPCMRadiiDerivativeComputeParameters_t structure that is packaged inside \\c parameters.\n Both maximum iterations and convergence threshold may be tuned based on the specific problem.\n\n The required temporary workspace must be allocated using the sizes returned by \\c cuestPCMRadiiDerivativeComputeWorkspaceQuery.\n The number of points, npoint, can be queried from the \\c cuestPCMIntPlan_t structure using the \\c CUEST_PCMINTPLAN_NUM_POINT query,\n and is the sum of the \\c numAngularPointsPerAtom array used to create the plan.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMRADIIDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMRADIIDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                      cuEST handle. Must not be NULL.\n @param[in]  plan                        PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                  PCM radii derivative parameters (opaque handle) constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[in]  temporaryWorkspace          Temporary workspace buffer for this operation. Must not be NULL.\n @param[in]  densityMatrix               Total (alpha + beta) AO basis density matrix (size: nao × nao) on the GPU. Must not be NULL.\n @param[in]  inQ                         Input guess charges (size: npoint) on the GPU; user should initialize this array to zeros if no guess is available. Must not be NULL.\n @param[out] outQ                        Converged charges (size: npoint) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n @param[out] outPCMResults               PCM results (opaque handle) constructed with \\c cuestResultsCreate. Must not be NULL.\n                                             Elements contained in this handle are overwritten by the calculation results.\n @param[out] outPCMRadiiGradient         Output PCM radii derivative (size: natoms) on the GPU. Must not be NULL.\n                                             Elements of this array are overwritten by the calculation results.\n\n It is important to note that the return status is \\c CUEST_STATUS_SUCCESS even if the PCM charges did not converge within the maximum number of iterations provided by \\c parameters.\n The caller should check the \\c converged member of the \\c outPCMResults handle to determine whether the PCM charges returned by \\c outQ are converged.\n\n In the case where \\c maxIterations==0 , no PCG updates of the charges are performed.\n The values provided in \\c inQ are used to evaluate the PCM radii gradient, and are copied to \\c outQ on return.\n In this mode, a residual is still evaluated and reported through \\c outPCMResults, but the charges themselves are not modified.\n\n Similarly, if the PCG procedure reaches the maximum number of iterations without satisfying the requested\n convergence threshold, the gradient is still computed using the resulting (partially converged) charges.\n It is the caller’s responsibility to inspect the convergence information in \\c outPCMResults and decide whether the result is acceptable.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success, including the case where the PCM charges did not converge within the maximum number of iterations provided by the \\c parameters;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type; including \\c parameters;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMRadiiDerivativeCompute(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMRadiiDerivativeComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMRadiiGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMRadiiDerivativeCompute
        .expect("cuEST function `cuestPCMRadiiDerivativeCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        densityMatrix,
        inQ,
        outQ,
        outPCMResults,
        outPCMRadiiGradient,
    )
}

#[doc = " @brief Query the temporary workspace required for a PCM radii gradient computation.\n @ingroup PCM\n\n Calculate the memory required for a PCM radii gradient computation, including the PCG solve for charges,\n returning host/device workspace buffer sizes in the output descriptor. The user arrays \\c densityMatrix, \\c inQ,\n \\c outQ, \\c outPCMResults, and \\c outPCMRadiiGradient may be NULL. All parameters and object handles must be valid.\n\n \\b Kernel and precision selection. This routine can use JIT-compiled kernels and/or reduced-precision\n \"ffloat\" compute, selected through the compute parameters:\n   - \\c CUEST_PCMRADIIDERIVATIVECOMPUTE_PARAMETERS_JIT_USAGE_MODE (\\c cuestJITUsageMode_t): \\c CUEST_JIT_USAGE_MODE_ON\n     uses JIT-compiled kernels, \\c CUEST_JIT_USAGE_MODE_OFF uses AOT-compiled kernels. JIT-compiled kernels\n     are only available in the cuEST CUDA 13 package; the default is \\c CUEST_JIT_USAGE_MODE_ON in the\n     cuEST CUDA 13 package and \\c CUEST_JIT_USAGE_MODE_OFF in the cuEST CUDA 12 package. Requesting\n     \\c CUEST_JIT_USAGE_MODE_ON with the cuEST CUDA 12 package returns \\c CUEST_STATUS_INVALID_ARGUMENT.\n   - \\c CUEST_PCMRADIIDERIVATIVECOMPUTE_PARAMETERS_FFLOAT_USAGE_MODE (\\c cuestFfloatUsageMode_t) selects the compute precision:\n     \\c CUEST_FFLOAT_USAGE_MODE_ON requests ffloat compute, \\c CUEST_FFLOAT_USAGE_MODE_OFF requests fp64\n     compute, and \\c CUEST_FFLOAT_USAGE_MODE_DEFAULT requests ffloat compute when it is available and\n     expected to provide a performance advantage on the present GPU architecture.\n   - JIT-compiled kernels and ffloat compute may be combined. When JIT is enabled, ffloat compute can be used\n     throughout the operation. When JIT is disabled, ffloat compute can be used only while solving for the\n     PCM charges; the remainder of the operation always uses fp64 compute.\n\n @param[in]  handle                        cuEST handle. Must not be NULL.\n @param[in]  plan                          PCM integral plan constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                    PCM radii derivative parameters constructed with \\c cuestParametersCreate. Must not be NULL.\n @param[out] temporaryWorkspaceDescriptor  Output descriptor for temporary workspace sizes. Must not be NULL.\n @param      densityMatrix                 Density matrix. May be NULL.\n @param      inQ                           Input Q vector. May be NULL.\n @param      outQ                          Output Q vector. May be NULL.\n @param      outPCMResults                 PCM results constructed with \\c cuestResultsCreate. May be NULL.\n @param      outPCMRadiiGradient           Output PCM radii gradient. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type; including \\c parameters;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if the \\c parameters are configured to have convergence threshold less than or equal to zero;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if a parameter value is invalid, or if JIT-compiled kernels are requested\n     with the cuEST CUDA 12 package;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMRadiiDerivativeComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMRadiiDerivativeComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    densityMatrix: *const f64,
    inQ: *const f64,
    outQ: *mut f64,
    outPCMResults: cuestPCMResults_t,
    outPCMRadiiGradient: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPCMRadiiDerivativeComputeWorkspaceQuery.expect("cuEST function `cuestPCMRadiiDerivativeComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, densityMatrix, inQ, outQ, outPCMResults, outPCMRadiiGradient)
}

#[doc = " @brief Compute PCM integration grid point coordinates.\n @ingroup PCMAdvanced\n @since v0.2.0\n\n This routine retrieves the Cartesian coordinates of each molecular grid point\n used to define the PCM cavity surface. These are the grid points used\n by the corresponding PCM integral plan.\n\n The resulting coordinates are written to a user-provided output buffer\n of shape (\\c npoint × 3), containing the x, y, z coordinates of each grid point.\n\n Temporary device workspace must be preallocated based on the size returned by\n \\c cuestPCMIntegrationGridComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters         Compute parameters (\\c cuestPCMIntegrationGridComputeParameters_t) Must not be NULL\n @param[in]  temporaryWorkspace Preallocated GPU workspace buffer. Must not be NULL.\n @param[out] outGridPoints      Output buffer for computed grid point coordinates (size: \\c npoint × 3) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle, plan, or parameters type is incorrect;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntegrationGridCompute(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMIntegrationGridComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outGridPoints: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMIntegrationGridCompute
        .expect("cuEST function `cuestPCMIntegrationGridCompute` not found in the loaded library")(
        handle,
        plan,
        parameters,
        temporaryWorkspace,
        outGridPoints,
    )
}

#[doc = " @brief Query temporary workspace requirements for PCM grid coordinate computation.\n @ingroup PCMAdvanced\n @since v0.2.0\n\n Determines the GPU memory workspace needed to generate the coordinates\n of molecular integration grid points defined by the provided PCM integral plan.\n This function must be called before \\c cuestPCMIntegrationGridCompute to ensure sufficient allocation.\n\n Only workspace sizes are computed; input and output pointers may be NULL.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  parameters                   Compute parameters (\\c cuestPCMIntegrationGridComputeParameters_t) Must not be NULL\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required workspace sizes (in bytes). Must not be NULL.\n @param      outGridPoints                Output array for grid point coordinates. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle, plan, or parameters type is incorrect;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntegrationGridComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    parameters: cuestPCMIntegrationGridComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outGridPoints: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPCMIntegrationGridComputeWorkspaceQuery.expect("cuEST function `cuestPCMIntegrationGridComputeWorkspaceQuery` not found in the loaded library")(handle, plan, parameters, temporaryWorkspaceDescriptor, outGridPoints)
}

#[doc = " @brief Compute molecular integration weights for PCM quadrature.\n @ingroup PCMAdvanced\n @since v0.2.0\n\n This routine computes the integration weights associated with each grid point\n in the molecular integration grid used for PCM integration.\n The weights returned depend on the selected weight type, specified by \\c weightType.\n\n Supported weighting schemes include:\n   - \\c CUEST_PCMINTEGRATIONWEIGHT_PARAMETERS_WEIGHTTYPE_SWITCHING — value of the iSWIG switching function;\n   - \\c CUEST_PCMINTEGRATIONWEIGHT_PARAMETERS_WEIGHTTYPE_ZETA — value of zeta at the grid point.\n\n The result is written to \\c outGridWeights (size: \\c npoint × 1).\n Temporary workspace must be preallocated based on\n \\c cuestPCMIntegrationWeightComputeWorkspaceQuery.\n\n @param[in]  handle             cuEST handle. Must not be NULL.\n @param[in]  plan               PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  weightType         Weight type to return (switching function value or zeta).\n @param[in]  parameters         Compute parameters (\\c cuestPCMIntegrationWeightComputeParameters_t) Must not be NULL\n @param[in]  temporaryWorkspace Preallocated GPU workspace buffer for temporary data. Must not be NULL.\n @param[out] outGridWeights     Output buffer for integration weights (size: \\c npoint × 1) on GPU. Must not be NULL.\n                                    Elements of this array are overwritten by the calculation results.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle, plan, or parameters type is invalid;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c weightType is unrecognized;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntegrationWeightCompute(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    weightType: cuestPCMIntegrationWeightComputeParametersWeightType_t,
    parameters: cuestPCMIntegrationWeightComputeParameters_t,
    temporaryWorkspace: *mut cuestWorkspace_t,
    outGridWeights: *mut f64,
) -> cuestStatus_t {
    dyload_lib()
        .cuestPCMIntegrationWeightCompute
        .expect("cuEST function `cuestPCMIntegrationWeightCompute` not found in the loaded library")(
        handle,
        plan,
        weightType,
        parameters,
        temporaryWorkspace,
        outGridWeights,
    )
}

#[doc = " @brief Query the temporary workspace required (in bytes) for computing PCM integration weights.\n @ingroup PCMAdvanced\n @since v0.2.0\n\n Determines GPU memory workspace requirements to compute integration weights\n on the molecular grid as defined by the provided PCM integral plan. The required\n workspace depends on the grid structure and selected weight type.\n\n This query should be executed before calling \\c cuestPCMIntegrationWeightCompute\n to ensure that sufficient device memory is available.\n\n @param[in]  handle                       cuEST handle. Must not be NULL.\n @param[in]  plan                         PCM integral plan (opaque handle) constructed with \\c cuestPCMIntPlanCreate. Must not be NULL.\n @param[in]  weightType                   Weight type to return (switching function value or zeta).\n @param[in]  parameters                   Compute parameters (\\c cuestPCMIntegrationWeightComputeParameters_t) Must not be NULL\n @param[out] temporaryWorkspaceDescriptor Output descriptor for required temporary workspace sizes (in bytes). Must not be NULL.\n @param      outGridWeights               Optional output buffer for computed weights. May be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if the handle, plan, or parameters type is incorrect;\n   - \\c CUEST_STATUS_INVALID_ARGUMENT if \\c weightType is unsupported;\n   - \\c CUEST_STATUS_EXCEPTION on device or internal error;\n   - \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestPCMIntegrationWeightComputeWorkspaceQuery(
    handle: cuestHandle_t,
    plan: cuestPCMIntPlan_t,
    weightType: cuestPCMIntegrationWeightComputeParametersWeightType_t,
    parameters: cuestPCMIntegrationWeightComputeParameters_t,
    temporaryWorkspaceDescriptor: *mut cuestWorkspaceDescriptor_t,
    outGridWeights: *mut f64,
) -> cuestStatus_t {
    dyload_lib().cuestPCMIntegrationWeightComputeWorkspaceQuery.expect("cuEST function `cuestPCMIntegrationWeightComputeWorkspaceQuery` not found in the loaded library")(handle, plan, weightType, parameters, temporaryWorkspaceDescriptor, outGridWeights)
}

#[doc = " @brief Query an attribute of a cuEST opaque handle.\n @ingroup Query\n\n The result pointer \\c attributeValue must reference sufficient memory for the type matching the specified attribute.\n Note that char* attributes are allocated internally and must be freed by the caller using free(). This allocation\n only occurs when the function completes successfully (returns \\c CUEST_STATUS_SUCCESS). On any error status no\n allocation is made and \\c attributeValue is left unchanged — cuEST never writes to it on a failing path — so a\n caller that pre-initializes \\c attributeValue (e.g. to \\c NULL) can rely on that value afterward for a\n deterministic cleanup pattern, and there is nothing to free.\n\n @param[in]  handle              cuEST handle. Must not be NULL.\n @param[in]  type                enum indicating which type of object is being queried\n @param[in]  object              cuEST object to be queried (opaque handle). Must not be NULL.\n @param[in]  attribute           Attribute/property to query. (Should be an enum appropriate to the type of object.)\n @param[out] attributeValue      Pointer to memory for result (type depends on attribute). Must not be NULL.\n @param[in]  attributeValueSize  Number of bytes in \\c attributeValue. Must match sizeof(type).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_INVALID_HANDLE if the cuEST handle is NULL;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c object or \\c attributeValue is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c object is not of the specified \\c type;\n   - \\c CUEST_STATUS_INVALID_SIZE if \\c attributeValueSize does not match \\c attribute type size;\n   - \\c CUEST_STATUS_INVALID_ATTRIBUTE if \\c attribute is not recognized;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for internal errors."]
pub unsafe fn cuestQuery(
    handle: cuestHandle_t,
    type_: cuestType_t,
    object: *const c_void,
    attribute: c_int,
    attributeValue: *mut c_void,
    attributeValueSize: usize,
) -> cuestStatus_t {
    dyload_lib().cuestQuery.expect("cuEST function `cuestQuery` not found in the loaded library")(
        handle,
        type_,
        object,
        attribute,
        attributeValue,
        attributeValueSize,
    )
}

#[doc = " @brief Create a new cuEST results object of the specified type.\n @ingroup Results\n\n Allocates and initializes an opaque results object of the type specified by \\c resultsType.\n The created object is returned via \\c outResults and must be destroyed with \\c cuestResultsDestroy.\n\n @param[in]  resultsType      Type of results object to create (see \\c cuestResultsType_t).\n @param[out] outResults       Pointer to receive allocated opaque results handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c outResults is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c resultsType is invalid;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for internal errors.\n\n @note On error, \\c *outResults is set to NULL."]
pub unsafe fn cuestResultsCreate(
    resultsType: cuestResultsType_t,
    outResults: *mut *mut c_void,
) -> cuestStatus_t {
    dyload_lib()
        .cuestResultsCreate
        .expect("cuEST function `cuestResultsCreate` not found in the loaded library")(
        resultsType,
        outResults,
    )
}

#[doc = " @brief Destroy and free a cuEST results object of the specified type.\n @ingroup Results\n\n Properly deallocates and cleans up resources for a results handle created by \\c cuestResultsCreate.\n\n @param[in] resultsType    Type of results object (must match creation).\n @param[in] results        Opaque results handle to destroy; must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c results is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c resultsType does not match the actual type of \\c results;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for internal errors."]
pub unsafe fn cuestResultsDestroy(
    resultsType: cuestResultsType_t,
    results: *mut c_void,
) -> cuestStatus_t {
    dyload_lib()
        .cuestResultsDestroy
        .expect("cuEST function `cuestResultsDestroy` not found in the loaded library")(
        resultsType,
        results,
    )
}

#[doc = " @brief Query the value of a given attribute from a cuEST results object.\n @ingroup Results\n\n Retrieves the value associated with an attribute for the given \\c results object. The type and meaning\n of \\c results and \\c resultValue depend on \\c resultsType. The result is placed in \\c resultValue, whose\n size must match the expected value size.\n\n @param[in]  resultsType      Type of results object being queried.\n @param[in]  results          Opaque handle of the results object (must not be NULL).\n @param[in]  attribute        Integer attribute identifier (cast to appropriate attribute enum internally).\n @param[out] resultValue      Destination for the queried value (must not be NULL).\n @param[in]  resultValueSize  Size in bytes of \\c resultValue buffer (must match queried type).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if \\c results or \\c resultValue is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c results is not of the specified \\c resultsType;\n   - \\c CUEST_STATUS_INVALID_SIZE if \\c resultValueSize does not match attribute type size;\n   - \\c CUEST_STATUS_INVALID_ATTRIBUTE if \\c attribute is not recognized;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR for internal errors."]
pub unsafe fn cuestResultsQuery(
    resultsType: cuestResultsType_t,
    results: *const c_void,
    attribute: c_int,
    resultValue: *mut c_void,
    resultValueSize: usize,
) -> cuestStatus_t {
    dyload_lib()
        .cuestResultsQuery
        .expect("cuEST function `cuestResultsQuery` not found in the loaded library")(
        resultsType,
        results,
        attribute,
        resultValue,
        resultValueSize,
    )
}

#[doc = " @brief Create a new cuEST parameters object of the specified type.\n @ingroup Parameters\n\n Allocates and initializes an opaque parameter object of the type specified by \\c parameterType.\n The created object is returned via \\c outParameters and must be destroyed with \\c cuestParametersDestroy.\n\n @param[in]  parameterType      Type of parameters object to create (see \\c cuestParametersType_t).\n @param[out] outParameters      Pointer to receive allocated opaque parameters handle. Must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if \\c parameterType is not a valid parameters type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestParametersCreate(
    parameterType: cuestParametersType_t,
    outParameters: *mut *mut c_void,
) -> cuestStatus_t {
    dyload_lib()
        .cuestParametersCreate
        .expect("cuEST function `cuestParametersCreate` not found in the loaded library")(
        parameterType,
        outParameters,
    )
}

#[doc = " @brief Destroy and free a cuEST parameters object of the specified type.\n @ingroup Parameters\n\n Properly deallocates and cleans up resources for a parameters handle created by \\c cuestParametersCreate.\n\n @param[in] parameterType    Type of parameters object (must match creation).\n @param[in] parameters       Opaque parameters handle to destroy; must not be NULL.\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestParametersDestroy(
    parameterType: cuestParametersType_t,
    parameters: *mut c_void,
) -> cuestStatus_t {
    dyload_lib()
        .cuestParametersDestroy
        .expect("cuEST function `cuestParametersDestroy` not found in the loaded library")(
        parameterType,
        parameters,
    )
}

#[doc = " @brief Query the value of a given attribute from a cuEST parameters object.\n @ingroup Parameters\n\n Retrieves the value associated with an attribute for the given \\c parameters object. The type and meaning\n of \\c attribute and \\c attributeValue depend on \\c parameterType. The result is placed in \\c attributeValue, whose\n size must match the expected value size.\n\n For string-valued attributes (e.g. \\c CUEST_HANDLE_PARAMETERS_JIT_CACHE_DIR, queried as a\n \\c char*), cuEST allocates the returned, NUL-terminated buffer with C's \\c malloc and the caller\n owns it: it must be released with C's native \\c free(). This allocation only occurs when the\n function completes successfully (returns \\c CUEST_STATUS_SUCCESS). On any error status no\n allocation is made and \\c attributeValue is left unchanged — cuEST never writes to it on a\n failing path — so a caller that pre-initializes \\c attributeValue (e.g. to \\c NULL) can rely on\n that value afterward for a deterministic cleanup pattern, and there is nothing to free.\n\n @param[in]  parameterType       Type of parameters object being queried.\n @param[in]  parameters          Opaque handle of the parameters object (must not be NULL).\n @param[in]  attribute           Integer attribute identifier (cast to appropriate attribute enum internally).\n @param[out] attributeValue      Destination for the queried value (must not be NULL).\n @param[in]  attributeValueSize  Size in bytes of \\c attributeValue buffer (must match queried type).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_SIZE if \\c attributeValueSize does not match the attribute type size;\n   - \\c CUEST_STATUS_INVALID_PARAMETER if the \\c attribute is not valid for the given \\c parameterType;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestParametersQuery(
    parameterType: cuestParametersType_t,
    parameters: *const c_void,
    attribute: c_int,
    attributeValue: *mut c_void,
    attributeValueSize: usize,
) -> cuestStatus_t {
    dyload_lib()
        .cuestParametersQuery
        .expect("cuEST function `cuestParametersQuery` not found in the loaded library")(
        parameterType,
        parameters,
        attribute,
        attributeValue,
        attributeValueSize,
    )
}

#[doc = " @brief Configure the value of a given attribute for a cuEST parameters object.\n @ingroup Parameters\n\n Sets or updates the value of an attribute for the given \\c parameters object. The type and semantics\n of the \\c attribute and \\c attributeValue depend on \\c parameterType. The input value is provided as a pointer,\n and its size must match the expected storage for the attribute.\n\n @param[in]  parameterType         Type of parameters object to configure.\n @param[in,out] parameters            Opaque handle to parameters object (must not be NULL).\n @param[in]  attribute             Integer attribute identifier (cast to appropriate attribute enum internally).\n @param[in]  attributeValue        Pointer to the new value (must not be NULL).\n @param[in]  attributeValueSize    Size in bytes of \\c attributeValue (must match attribute type).\n\n @return\n   - \\c CUEST_STATUS_SUCCESS on success;\n   - \\c CUEST_STATUS_NULL_POINTER if any required pointer is NULL;\n   - \\c CUEST_STATUS_INVALID_TYPE if opaque handles are not the correct type;\n   - \\c CUEST_STATUS_INVALID_SIZE if \\c attributeValueSize does not match the attribute type size;\n   - \\c CUEST_STATUS_INVALID_PARAMETER if the \\c attribute is not valid for the given \\c parameterType;\n   - \\c CUEST_STATUS_EXCEPTION or \\c CUEST_STATUS_UNKNOWN_ERROR otherwise."]
pub unsafe fn cuestParametersConfigure(
    parameterType: cuestParametersType_t,
    parameters: *mut c_void,
    attribute: c_int,
    attributeValue: *const c_void,
    attributeValueSize: usize,
) -> cuestStatus_t {
    dyload_lib()
        .cuestParametersConfigure
        .expect("cuEST function `cuestParametersConfigure` not found in the loaded library")(
        parameterType,
        parameters,
        attribute,
        attributeValue,
        attributeValueSize,
    )
}
