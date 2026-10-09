//! Library initializer implementation for dynamic loading.
//!
//! This file is generated automatically by `scripts/generate_ffi.py`; do not
//! edit it manually.

use super::*;
use libloading::{Library, Symbol};

unsafe fn get_symbol<'f, F>(libs: &'f [Library], name: &[u8]) -> Option<Symbol<'f, F>> {
    libs.iter().find_map(|lib| lib.get::<F>(name).ok())
}

impl DyLoadLib {
    pub unsafe fn new(libs: Vec<libloading::Library>, libs_path: Vec<String>) -> DyLoadLib {
        let mut result = DyLoadLib {
            __libraries: vec![],      // dummy, set later
            __libraries_path: vec![], // dummy, set later
            __error: None,
            cuestCreate: get_symbol(&libs, b"cuestCreate\0").map(|sym| *sym),
            cuestDestroy: get_symbol(&libs, b"cuestDestroy\0").map(|sym| *sym),
            cuestSetMathMode: get_symbol(&libs, b"cuestSetMathMode\0").map(|sym| *sym),
            cuestGetMathMode: get_symbol(&libs, b"cuestGetMathMode\0").map(|sym| *sym),
            cuestSetComputeCapabilityTarget: get_symbol(
                &libs,
                b"cuestSetComputeCapabilityTarget\0",
            )
            .map(|sym| *sym),
            cuestGetComputeCapabilityTarget: get_symbol(
                &libs,
                b"cuestGetComputeCapabilityTarget\0",
            )
            .map(|sym| *sym),
            cuestGetMajorVersion: get_symbol(&libs, b"cuestGetMajorVersion\0").map(|sym| *sym),
            cuestGetMinorVersion: get_symbol(&libs, b"cuestGetMinorVersion\0").map(|sym| *sym),
            cuestGetPatchVersion: get_symbol(&libs, b"cuestGetPatchVersion\0").map(|sym| *sym),
            cuestAOShellCreate: get_symbol(&libs, b"cuestAOShellCreate\0").map(|sym| *sym),
            cuestAOShellDestroy: get_symbol(&libs, b"cuestAOShellDestroy\0").map(|sym| *sym),
            cuestAOBasisCreate: get_symbol(&libs, b"cuestAOBasisCreate\0").map(|sym| *sym),
            cuestAOBasisCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestAOBasisCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestAOBasisDestroy: get_symbol(&libs, b"cuestAOBasisDestroy\0").map(|sym| *sym),
            cuestAOPairListCreate: get_symbol(&libs, b"cuestAOPairListCreate\0").map(|sym| *sym),
            cuestAOPairListCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestAOPairListCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestAOPairListDestroy: get_symbol(&libs, b"cuestAOPairListDestroy\0").map(|sym| *sym),
            cuestECPCompute: get_symbol(&libs, b"cuestECPCompute\0").map(|sym| *sym),
            cuestECPComputeWorkspaceQuery: get_symbol(&libs, b"cuestECPComputeWorkspaceQuery\0")
                .map(|sym| *sym),
            cuestECPDerivativeCompute: get_symbol(&libs, b"cuestECPDerivativeCompute\0")
                .map(|sym| *sym),
            cuestECPDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestECPDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestECPAtomCreate: get_symbol(&libs, b"cuestECPAtomCreate\0").map(|sym| *sym),
            cuestECPAtomDestroy: get_symbol(&libs, b"cuestECPAtomDestroy\0").map(|sym| *sym),
            cuestECPShellCreate: get_symbol(&libs, b"cuestECPShellCreate\0").map(|sym| *sym),
            cuestECPShellDestroy: get_symbol(&libs, b"cuestECPShellDestroy\0").map(|sym| *sym),
            cuestECPIntPlanCreate: get_symbol(&libs, b"cuestECPIntPlanCreate\0").map(|sym| *sym),
            cuestECPIntPlanCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestECPIntPlanCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestECPIntPlanDestroy: get_symbol(&libs, b"cuestECPIntPlanDestroy\0").map(|sym| *sym),
            cuestOEIntPlanCreate: get_symbol(&libs, b"cuestOEIntPlanCreate\0").map(|sym| *sym),
            cuestOEIntPlanCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestOEIntPlanCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestOEIntPlanDestroy: get_symbol(&libs, b"cuestOEIntPlanDestroy\0").map(|sym| *sym),
            cuestDFIntPlanCreate: get_symbol(&libs, b"cuestDFIntPlanCreate\0").map(|sym| *sym),
            cuestDFIntPlanCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFIntPlanCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFIntPlanDestroy: get_symbol(&libs, b"cuestDFIntPlanDestroy\0").map(|sym| *sym),
            cuestOverlapCompute: get_symbol(&libs, b"cuestOverlapCompute\0").map(|sym| *sym),
            cuestOverlapComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestOverlapComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestOverlapDerivativeCompute: get_symbol(&libs, b"cuestOverlapDerivativeCompute\0")
                .map(|sym| *sym),
            cuestOverlapDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestOverlapDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestKineticCompute: get_symbol(&libs, b"cuestKineticCompute\0").map(|sym| *sym),
            cuestKineticComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestKineticComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestKineticDerivativeCompute: get_symbol(&libs, b"cuestKineticDerivativeCompute\0")
                .map(|sym| *sym),
            cuestKineticDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestKineticDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestMultipoleCompute: get_symbol(&libs, b"cuestMultipoleCompute\0").map(|sym| *sym),
            cuestMultipoleComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestMultipoleComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestMultipoleDerivativeCompute: get_symbol(
                &libs,
                b"cuestMultipoleDerivativeCompute\0",
            )
            .map(|sym| *sym),
            cuestMultipoleDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestMultipoleDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestAngularMomentumCompute: get_symbol(&libs, b"cuestAngularMomentumCompute\0")
                .map(|sym| *sym),
            cuestAngularMomentumComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestAngularMomentumComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestAngularMomentumDerivativeCompute: get_symbol(
                &libs,
                b"cuestAngularMomentumDerivativeCompute\0",
            )
            .map(|sym| *sym),
            cuestAngularMomentumDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestAngularMomentumDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNablaCompute: get_symbol(&libs, b"cuestNablaCompute\0").map(|sym| *sym),
            cuestNablaComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNablaComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNablaDerivativeCompute: get_symbol(&libs, b"cuestNablaDerivativeCompute\0")
                .map(|sym| *sym),
            cuestNablaDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNablaDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPotentialCompute: get_symbol(&libs, b"cuestPotentialCompute\0").map(|sym| *sym),
            cuestPotentialComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPotentialComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPotentialDerivativeCompute: get_symbol(
                &libs,
                b"cuestPotentialDerivativeCompute\0",
            )
            .map(|sym| *sym),
            cuestPotentialDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPotentialDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFCoulombCompute: get_symbol(&libs, b"cuestDFCoulombCompute\0").map(|sym| *sym),
            cuestDFCoulombComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFCoulombComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFSymmetricExchangeCompute: get_symbol(
                &libs,
                b"cuestDFSymmetricExchangeCompute\0",
            )
            .map(|sym| *sym),
            cuestDFSymmetricExchangeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFSymmetricExchangeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFNonsymmetricExchangeCompute: get_symbol(
                &libs,
                b"cuestDFNonsymmetricExchangeCompute\0",
            )
            .map(|sym| *sym),
            cuestDFNonsymmetricExchangeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFNonsymmetricExchangeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFSymmetricDerivativeCompute: get_symbol(
                &libs,
                b"cuestDFSymmetricDerivativeCompute\0",
            )
            .map(|sym| *sym),
            cuestDFSymmetricDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFSymmetricDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestDFMOIntegralsCompute: get_symbol(&libs, b"cuestDFMOIntegralsCompute\0")
                .map(|sym| *sym),
            cuestDFMOIntegralsComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestDFMOIntegralsComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestAtomGridCreate: get_symbol(&libs, b"cuestAtomGridCreate\0").map(|sym| *sym),
            cuestAtomGridDestroy: get_symbol(&libs, b"cuestAtomGridDestroy\0").map(|sym| *sym),
            cuestMolecularGridCreate: get_symbol(&libs, b"cuestMolecularGridCreate\0")
                .map(|sym| *sym),
            cuestMolecularGridCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestMolecularGridCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestMolecularGridDestroy: get_symbol(&libs, b"cuestMolecularGridDestroy\0")
                .map(|sym| *sym),
            cuestXCIntPlanCreate: get_symbol(&libs, b"cuestXCIntPlanCreate\0").map(|sym| *sym),
            cuestXCIntPlanCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCIntPlanCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCIntPlanDestroy: get_symbol(&libs, b"cuestXCIntPlanDestroy\0").map(|sym| *sym),
            cuestXCIntegrationGridCompute: get_symbol(&libs, b"cuestXCIntegrationGridCompute\0")
                .map(|sym| *sym),
            cuestXCIntegrationGridComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCIntegrationGridComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCIntegrationWeightCompute: get_symbol(
                &libs,
                b"cuestXCIntegrationWeightCompute\0",
            )
            .map(|sym| *sym),
            cuestXCIntegrationWeightComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCIntegrationWeightComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCPotentialRKSCompute: get_symbol(&libs, b"cuestXCPotentialRKSCompute\0")
                .map(|sym| *sym),
            cuestXCPotentialRKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCPotentialRKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCPotentialUKSCompute: get_symbol(&libs, b"cuestXCPotentialUKSCompute\0")
                .map(|sym| *sym),
            cuestXCPotentialUKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCPotentialUKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCDensityCompute: get_symbol(&libs, b"cuestXCDensityCompute\0").map(|sym| *sym),
            cuestXCDensityComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCDensityComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCPotentialCompute: get_symbol(&libs, b"cuestXCPotentialCompute\0")
                .map(|sym| *sym),
            cuestXCPotentialComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCPotentialComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCNonsymmetricDensityCompute: get_symbol(
                &libs,
                b"cuestXCNonsymmetricDensityCompute\0",
            )
            .map(|sym| *sym),
            cuestXCNonsymmetricDensityComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCNonsymmetricDensityComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCDerivativeRKSCompute: get_symbol(&libs, b"cuestXCDerivativeRKSCompute\0")
                .map(|sym| *sym),
            cuestXCDerivativeRKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCDerivativeRKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCDerivativeUKSCompute: get_symbol(&libs, b"cuestXCDerivativeUKSCompute\0")
                .map(|sym| *sym),
            cuestXCDerivativeUKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCDerivativeUKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCDerivativeCompute: get_symbol(&libs, b"cuestXCDerivativeCompute\0")
                .map(|sym| *sym),
            cuestXCDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCGridDerivativeCompute: get_symbol(&libs, b"cuestXCGridDerivativeCompute\0")
                .map(|sym| *sym),
            cuestXCGridDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCGridDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestXCNonsymmetricDerivativeCompute: get_symbol(
                &libs,
                b"cuestXCNonsymmetricDerivativeCompute\0",
            )
            .map(|sym| *sym),
            cuestXCNonsymmetricDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestXCNonsymmetricDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCPotentialRKSCompute: get_symbol(
                &libs,
                b"cuestNonlocalXCPotentialRKSCompute\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCPotentialRKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNonlocalXCPotentialRKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCPotentialUKSCompute: get_symbol(
                &libs,
                b"cuestNonlocalXCPotentialUKSCompute\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCPotentialUKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNonlocalXCPotentialUKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCDerivativeRKSCompute: get_symbol(
                &libs,
                b"cuestNonlocalXCDerivativeRKSCompute\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNonlocalXCDerivativeRKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCDerivativeUKSCompute: get_symbol(
                &libs,
                b"cuestNonlocalXCDerivativeUKSCompute\0",
            )
            .map(|sym| *sym),
            cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestNonlocalXCDerivativeUKSComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMIntPlanCreate: get_symbol(&libs, b"cuestPCMIntPlanCreate\0").map(|sym| *sym),
            cuestPCMIntPlanCreateWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMIntPlanCreateWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMIntPlanDestroy: get_symbol(&libs, b"cuestPCMIntPlanDestroy\0").map(|sym| *sym),
            cuestPCMPotentialCompute: get_symbol(&libs, b"cuestPCMPotentialCompute\0")
                .map(|sym| *sym),
            cuestPCMPotentialComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMPotentialComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMDerivativeCompute: get_symbol(&libs, b"cuestPCMDerivativeCompute\0")
                .map(|sym| *sym),
            cuestPCMDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMRadiiDerivativeCompute: get_symbol(&libs, b"cuestPCMRadiiDerivativeCompute\0")
                .map(|sym| *sym),
            cuestPCMRadiiDerivativeComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMRadiiDerivativeComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMIntegrationGridCompute: get_symbol(&libs, b"cuestPCMIntegrationGridCompute\0")
                .map(|sym| *sym),
            cuestPCMIntegrationGridComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMIntegrationGridComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestPCMIntegrationWeightCompute: get_symbol(
                &libs,
                b"cuestPCMIntegrationWeightCompute\0",
            )
            .map(|sym| *sym),
            cuestPCMIntegrationWeightComputeWorkspaceQuery: get_symbol(
                &libs,
                b"cuestPCMIntegrationWeightComputeWorkspaceQuery\0",
            )
            .map(|sym| *sym),
            cuestQuery: get_symbol(&libs, b"cuestQuery\0").map(|sym| *sym),
            cuestResultsCreate: get_symbol(&libs, b"cuestResultsCreate\0").map(|sym| *sym),
            cuestResultsDestroy: get_symbol(&libs, b"cuestResultsDestroy\0").map(|sym| *sym),
            cuestResultsQuery: get_symbol(&libs, b"cuestResultsQuery\0").map(|sym| *sym),
            cuestParametersCreate: get_symbol(&libs, b"cuestParametersCreate\0").map(|sym| *sym),
            cuestParametersDestroy: get_symbol(&libs, b"cuestParametersDestroy\0").map(|sym| *sym),
            cuestParametersQuery: get_symbol(&libs, b"cuestParametersQuery\0").map(|sym| *sym),
            cuestParametersConfigure: get_symbol(&libs, b"cuestParametersConfigure\0")
                .map(|sym| *sym),
        };
        result.__libraries = libs;
        result.__libraries_path = libs_path;
        result
    }
}
