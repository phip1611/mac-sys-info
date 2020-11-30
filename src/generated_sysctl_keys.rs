/*
MIT License

Copyright (c) 2020 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Raw mappings to the sysctl values/keys. (The output of `$ sysctl -a`)
//! Last updated: 2020-11-26 on intel Macbook Pro on
//! MacOS Catalina 10.15.7
//!
//! Execute the binary `sysctl_output_keys_to_rust_code.rs` and copy
//! the output of this command to renew this code.
//!
//! This code uses [`derive_more::Display`]. If each enum variant
//! differs from the default impl (and has its own `#[display(fmt = "...")]`-macro
//! the build time takes up to one minute for the thousand of enum variants.
//! This is currently not the case but maybe in the future. The code
//! generation script has an option for that.

use derive_more::Display;

// If you want to change the code, change the code in `sysctl_output_keys_to_rust_code.rs`!
// Then delete the below, generate new code, and copy it here.
// #################################################################

/// This enum was generated using the binary `sysctl_output_keys_to_rust_code.rs`
/// It contains (hopefully) all keys that the output of `$ sysctl -a` can show.
/// This includes information about the CPU, the number of cores and caches.
/// Might make trouble/inconsistencies with the AppleSi-Macbooks. I can't test it yet.
#[derive(Debug, Display, PartialEq, Copy, Clone, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum SysctlKey {
    /// Key for 'audit.session.member_clear_sflags_mask'
    AuditSessionMember_clear_sflags_mask,
    /// Key for 'audit.session.member_set_sflags_mask'
    AuditSessionMember_set_sflags_mask,
    /// Key for 'audit.session.superuser_clear_sflags_mask'
    AuditSessionSuperuser_clear_sflags_mask,
    /// Key for 'audit.session.superuser_set_sflags_mask'
    AuditSessionSuperuser_set_sflags_mask,
    /// Key for 'debug.acpi_flags'
    DebugAcpi_flags,
    /// Key for 'debug.acpi_layer'
    DebugAcpi_layer,
    /// Key for 'debug.acpi_level'
    DebugAcpi_level,
    /// Key for 'debug.agpm.LogLevel'
    DebugAgpmLogLevel,
    /// Key for 'debug.batman'
    DebugBatman,
    /// Key for 'debug.bpf_bufsize'
    DebugBpf_bufsize,
    /// Key for 'debug.bpf_debug'
    DebugBpf_debug,
    /// Key for 'debug.bpf_maxbufsize'
    DebugBpf_maxbufsize,
    /// Key for 'debug.bpf_maxdevices'
    DebugBpf_maxdevices,
    /// Key for 'debug.bpf_wantpktap'
    DebugBpf_wantpktap,
    /// Key for 'debug.brcmfirewirelog'
    DebugBrcmfirewirelog,
    /// Key for 'debug.brcmlinkdebug'
    DebugBrcmlinkdebug,
    /// Key for 'debug.brcmlogging'
    DebugBrcmlogging,
    /// Key for 'debug.darkwake'
    DebugDarkwake,
    /// Key for 'debug.intel.dtraceEnable'
    DebugIntelDtraceEnable,
    /// Key for 'debug.intel.flipCount'
    DebugIntelFlipCount,
    /// Key for 'debug.intel.GlobalUsageTotal_Busy_nSec'
    DebugIntelGlobalUsageTotal_Busy_nSec,
    /// Key for 'debug.intel.GlobalUsageTotal_nSec'
    DebugIntelGlobalUsageTotal_nSec,
    /// Key for 'debug.intel.gpuUsageEnables'
    DebugIntelGpuUsageEnables,
    /// Key for 'debug.intel.gpuUsageEnablesCheck'
    DebugIntelGpuUsageEnablesCheck,
    /// Key for 'debug.intel.graphicsTracePointEnable'
    DebugIntelGraphicsTracePointEnable,
    /// Key for 'debug.intel.IGInterruptControl'
    DebugIntelIGInterruptControl,
    /// Key for 'debug.intel.kdctlVersion'
    DebugIntelKdctlVersion,
    /// Key for 'debug.intel.mSecCalcGPUBusy'
    DebugIntelMSecCalcGPUBusy,
    /// Key for 'debug.intel.oaEnable'
    DebugIntelOaEnable,
    /// Key for 'debug.intel.perfEventEnable'
    DebugIntelPerfEventEnable,
    /// Key for 'debug.intel.ringBlitUsage'
    DebugIntelRingBlitUsage,
    /// Key for 'debug.intel.ringBlit_nSec'
    DebugIntelRingBlit_nSec,
    /// Key for 'debug.intel.ringMainUsage'
    DebugIntelRingMainUsage,
    /// Key for 'debug.intel.ringMain_nSec'
    DebugIntelRingMain_nSec,
    /// Key for 'debug.intel.ringMediaUsage'
    DebugIntelRingMediaUsage,
    /// Key for 'debug.intel.ringMedia_nSec'
    DebugIntelRingMedia_nSec,
    /// Key for 'debug.intel.ringOnSample'
    DebugIntelRingOnSample,
    /// Key for 'debug.intel.ringTakeSample'
    DebugIntelRingTakeSample,
    /// Key for 'debug.intel.ringVEBoxUsage'
    DebugIntelRingVEBoxUsage,
    /// Key for 'debug.intel.ringVEBox_nSec'
    DebugIntelRingVEBox_nSec,
    /// Key for 'debug.intel.schedEnableThrottleOverride'
    DebugIntelSchedEnableThrottleOverride,
    /// Key for 'debug.intel.schedPriCreditsHigh'
    DebugIntelSchedPriCreditsHigh,
    /// Key for 'debug.intel.schedPriCreditsLow'
    DebugIntelSchedPriCreditsLow,
    /// Key for 'debug.intel.schedPriCreditsNormal'
    DebugIntelSchedPriCreditsNormal,
    /// Key for 'debug.intel.schedPriCreditsNormalHigh'
    DebugIntelSchedPriCreditsNormalHigh,
    /// Key for 'debug.intel.schedPriElevatePID'
    DebugIntelSchedPriElevatePID,
    /// Key for 'debug.intel.schedPriPreemption'
    DebugIntelSchedPriPreemption,
    /// Key for 'debug.intel.schedThrottleHighPriByVal'
    DebugIntelSchedThrottleHighPriByVal,
    /// Key for 'debug.intel.schedThrottleLowPriByVal'
    DebugIntelSchedThrottleLowPriByVal,
    /// Key for 'debug.intel.schedThrottleNormalHighPriByVal'
    DebugIntelSchedThrottleNormalHighPriByVal,
    /// Key for 'debug.intel.schedThrottleNormalPriByVal'
    DebugIntelSchedThrottleNormalPriByVal,
    /// Key for 'debug.intel.swapCount'
    DebugIntelSwapCount,
    /// Key for 'debug.intel.telemetryAltConfig'
    DebugIntelTelemetryAltConfig,
    /// Key for 'debug.intel.telemetryConfig'
    DebugIntelTelemetryConfig,
    /// Key for 'debug.intel.telemetryMode'
    DebugIntelTelemetryMode,
    /// Key for 'debug.intel.telemetryNumFrame'
    DebugIntelTelemetryNumFrame,
    /// Key for 'debug.intel.telemetrySampleLocations'
    DebugIntelTelemetrySampleLocations,
    /// Key for 'debug.intel.telemetrySpot1'
    DebugIntelTelemetrySpot1,
    /// Key for 'debug.intel.telemetryStartFrame'
    DebugIntelTelemetryStartFrame,
    /// Key for 'debug.intel.telemetryStatPasses'
    DebugIntelTelemetryStatPasses,
    /// Key for 'debug.intel.telemetryStopFrame'
    DebugIntelTelemetryStopFrame,
    /// Key for 'debug.intel.telemetryTestCase'
    DebugIntelTelemetryTestCase,
    /// Key for 'debug.intel.telemetryUsageReportmSec'
    DebugIntelTelemetryUsageReportmSec,
    /// Key for 'debug.intel.telemetryVersion'
    DebugIntelTelemetryVersion,
    /// Key for 'debug.intel.temp0'
    DebugIntelTemp0,
    /// Key for 'debug.intel.temp1'
    DebugIntelTemp1,
    /// Key for 'debug.intel.temp2'
    DebugIntelTemp2,
    /// Key for 'debug.intel.temp3'
    DebugIntelTemp3,
    /// Key for 'debug.intel.temp4'
    DebugIntelTemp4,
    /// Key for 'debug.intelfb.EUCount'
    DebugIntelfbEUCount,
    /// Key for 'debug.intelfb.fLastRequestedPState'
    DebugIntelfbFLastRequestedPState,
    /// Key for 'debug.intelfb.FakeType2Dongle'
    DebugIntelfbFakeType2Dongle,
    /// Key for 'debug.intelfb.forceSlicesGTx'
    DebugIntelfbForceSlicesGTx,
    /// Key for 'debug.intelfb.graphicsTracePointEnable'
    DebugIntelfbGraphicsTracePointEnable,
    /// Key for 'debug.intelfb.IGInterruptControl'
    DebugIntelfbIGInterruptControl,
    /// Key for 'debug.intelfb.sliceInfo'
    DebugIntelfbSliceInfo,
    /// Key for 'debug.intelfb.temp0'
    DebugIntelfbTemp0,
    /// Key for 'debug.intelfb.temp1'
    DebugIntelfbTemp1,
    /// Key for 'debug.intelfb.temp2'
    DebugIntelfbTemp2,
    /// Key for 'debug.intelfb.temp3'
    DebugIntelfbTemp3,
    /// Key for 'debug.intelfb.temp4'
    DebugIntelfbTemp4,
    /// Key for 'debug.intelfb.testCase'
    DebugIntelfbTestCase,
    /// Key for 'debug.iokit'
    DebugIokit,
    /// Key for 'debug.ioppf'
    DebugIoppf,
    /// Key for 'debug.iotrace'
    DebugIotrace,
    /// Key for 'debug.kextlog'
    DebugKextlog,
    /// Key for 'debug.lowpri_throttle_enabled'
    DebugLowpri_throttle_enabled,
    /// Key for 'debug.lowpri_throttle_max_iosize'
    DebugLowpri_throttle_max_iosize,
    /// Key for 'debug.lowpri_throttle_tier1_io_period_msecs'
    DebugLowpri_throttle_tier1_io_period_msecs,
    /// Key for 'debug.lowpri_throttle_tier1_io_period_ssd_msecs'
    DebugLowpri_throttle_tier1_io_period_ssd_msecs,
    /// Key for 'debug.lowpri_throttle_tier1_window_msecs'
    DebugLowpri_throttle_tier1_window_msecs,
    /// Key for 'debug.lowpri_throttle_tier2_io_period_msecs'
    DebugLowpri_throttle_tier2_io_period_msecs,
    /// Key for 'debug.lowpri_throttle_tier2_io_period_ssd_msecs'
    DebugLowpri_throttle_tier2_io_period_ssd_msecs,
    /// Key for 'debug.lowpri_throttle_tier2_window_msecs'
    DebugLowpri_throttle_tier2_window_msecs,
    /// Key for 'debug.lowpri_throttle_tier3_io_period_msecs'
    DebugLowpri_throttle_tier3_io_period_msecs,
    /// Key for 'debug.lowpri_throttle_tier3_io_period_ssd_msecs'
    DebugLowpri_throttle_tier3_io_period_ssd_msecs,
    /// Key for 'debug.lowpri_throttle_tier3_window_msecs'
    DebugLowpri_throttle_tier3_window_msecs,
    /// Key for 'debug.noidle'
    DebugNoidle,
    /// Key for 'debug.sched'
    DebugSched,
    /// Key for 'debug.swd_panic'
    DebugSwd_panic,
    /// Key for 'debug.swd_sleep_timeout'
    DebugSwd_sleep_timeout,
    /// Key for 'debug.swd_timeout'
    DebugSwd_timeout,
    /// Key for 'debug.swd_wake_timeout'
    DebugSwd_wake_timeout,
    /// Key for 'debug.toggle_address_reuse'
    DebugToggle_address_reuse,
    /// Key for 'debug.wlan_ll_debug'
    DebugWlan_ll_debug,
    /// Key for 'hw.activecpu'
    HwActivecpu,
    /// Key for 'hw.busfrequency'
    HwBusfrequency,
    /// Key for 'hw.busfrequency_max'
    HwBusfrequency_max,
    /// Key for 'hw.busfrequency_min'
    HwBusfrequency_min,
    /// Key for 'hw.byteorder'
    HwByteorder,
    /// Key for 'hw.cacheconfig'
    HwCacheconfig,
    /// Key for 'hw.cachelinesize'
    HwCachelinesize,
    /// Key for 'hw.cachesize'
    HwCachesize,
    /// Key for 'hw.cpu64bit_capable'
    HwCpu64bit_capable,
    /// Key for 'hw.cpufamily'
    HwCpufamily,
    /// Key for 'hw.cpufrequency'
    HwCpufrequency,
    /// Key for 'hw.cpufrequency_max'
    HwCpufrequency_max,
    /// Key for 'hw.cpufrequency_min'
    HwCpufrequency_min,
    /// Key for 'hw.cpusubtype'
    HwCpusubtype,
    /// Key for 'hw.cputhreadtype'
    HwCputhreadtype,
    /// Key for 'hw.cputype'
    HwCputype,
    /// Key for 'hw.l1dcachesize'
    HwL1dcachesize,
    /// Key for 'hw.l1icachesize'
    HwL1icachesize,
    /// Key for 'hw.l2cachesize'
    HwL2cachesize,
    /// Key for 'hw.l3cachesize'
    HwL3cachesize,
    /// Key for 'hw.logicalcpu'
    HwLogicalcpu,
    /// Key for 'hw.logicalcpu_max'
    HwLogicalcpu_max,
    /// Key for 'hw.memsize'
    HwMemsize,
    /// Key for 'hw.ncpu'
    HwNcpu,
    /// Key for 'hw.optional.adx'
    HwOptionalAdx,
    /// Key for 'hw.optional.aes'
    HwOptionalAes,
    /// Key for 'hw.optional.avx1_0'
    HwOptionalAvx1_0,
    /// Key for 'hw.optional.avx2_0'
    HwOptionalAvx2_0,
    /// Key for 'hw.optional.avx512bw'
    HwOptionalAvx512bw,
    /// Key for 'hw.optional.avx512cd'
    HwOptionalAvx512cd,
    /// Key for 'hw.optional.avx512dq'
    HwOptionalAvx512dq,
    /// Key for 'hw.optional.avx512f'
    HwOptionalAvx512f,
    /// Key for 'hw.optional.avx512ifma'
    HwOptionalAvx512ifma,
    /// Key for 'hw.optional.avx512vbmi'
    HwOptionalAvx512vbmi,
    /// Key for 'hw.optional.avx512vl'
    HwOptionalAvx512vl,
    /// Key for 'hw.optional.bmi1'
    HwOptionalBmi1,
    /// Key for 'hw.optional.bmi2'
    HwOptionalBmi2,
    /// Key for 'hw.optional.enfstrg'
    HwOptionalEnfstrg,
    /// Key for 'hw.optional.f16c'
    HwOptionalF16c,
    /// Key for 'hw.optional.floatingpoint'
    HwOptionalFloatingpoint,
    /// Key for 'hw.optional.fma'
    HwOptionalFma,
    /// Key for 'hw.optional.hle'
    HwOptionalHle,
    /// Key for 'hw.optional.mmx'
    HwOptionalMmx,
    /// Key for 'hw.optional.mpx'
    HwOptionalMpx,
    /// Key for 'hw.optional.rdrand'
    HwOptionalRdrand,
    /// Key for 'hw.optional.rtm'
    HwOptionalRtm,
    /// Key for 'hw.optional.sgx'
    HwOptionalSgx,
    /// Key for 'hw.optional.sse'
    HwOptionalSse,
    /// Key for 'hw.optional.sse2'
    HwOptionalSse2,
    /// Key for 'hw.optional.sse3'
    HwOptionalSse3,
    /// Key for 'hw.optional.sse4_1'
    HwOptionalSse4_1,
    /// Key for 'hw.optional.sse4_2'
    HwOptionalSse4_2,
    /// Key for 'hw.optional.supplementalsse3'
    HwOptionalSupplementalsse3,
    /// Key for 'hw.optional.x86_64'
    HwOptionalX86_64,
    /// Key for 'hw.packages'
    HwPackages,
    /// Key for 'hw.pagesize'
    HwPagesize,
    /// Key for 'hw.pagesize32'
    HwPagesize32,
    /// Key for 'hw.physicalcpu'
    HwPhysicalcpu,
    /// Key for 'hw.physicalcpu_max'
    HwPhysicalcpu_max,
    /// Key for 'hw.targettype'
    HwTargettype,
    /// Key for 'hw.tbfrequency'
    HwTbfrequency,
    /// Key for 'kern.affinity_sets_enabled'
    KernAffinity_sets_enabled,
    /// Key for 'kern.affinity_sets_mapping'
    KernAffinity_sets_mapping,
    /// Key for 'kern.aiomax'
    KernAiomax,
    /// Key for 'kern.aioprocmax'
    KernAioprocmax,
    /// Key for 'kern.aiothreads'
    KernAiothreads,
    /// Key for 'kern.aotmode'
    KernAotmode,
    /// Key for 'kern.aotmodebits'
    KernAotmodebits,
    /// Key for 'kern.argmax'
    KernArgmax,
    /// Key for 'kern.bootargs'
    KernBootargs,
    /// Key for 'kern.bootsessionuuid'
    KernBootsessionuuid,
    /// Key for 'kern.bootsignature'
    KernBootsignature,
    /// Key for 'kern.boottime'
    KernBoottime,
    /// Key for 'kern.check_openevt'
    KernCheck_openevt,
    /// Key for 'kern.clockrate'
    KernClockrate,
    /// Key for 'kern.consoleoptions'
    KernConsoleoptions,
    /// Key for 'kern.coredump'
    KernCoredump,
    /// Key for 'kern.corefile'
    KernCorefile,
    /// Key for 'kern.delayterm'
    KernDelayterm,
    /// Key for 'kern.ds_supgroups_supported'
    KernDs_supgroups_supported,
    /// Key for 'kern.dtrace.buffer_memory_inuse'
    KernDtraceBuffer_memory_inuse,
    /// Key for 'kern.dtrace.buffer_memory_maxsize'
    KernDtraceBuffer_memory_maxsize,
    /// Key for 'kern.dtrace.difo_maxsize'
    KernDtraceDifo_maxsize,
    /// Key for 'kern.dtrace.dof_maxsize'
    KernDtraceDof_maxsize,
    /// Key for 'kern.dtrace.dof_mode'
    KernDtraceDof_mode,
    /// Key for 'kern.dtrace.err_verbose'
    KernDtraceErr_verbose,
    /// Key for 'kern.dtrace.global_maxsize'
    KernDtraceGlobal_maxsize,
    /// Key for 'kern.dtrace.ignore_fbt_blacklist'
    KernDtraceIgnore_fbt_blacklist,
    /// Key for 'kern.dtrace.provide_private_probes'
    KernDtraceProvide_private_probes,
    /// Key for 'kern.eventhandler.debug'
    KernEventhandlerDebug,
    /// Key for 'kern.flush_cache_on_write'
    KernFlush_cache_on_write,
    /// Key for 'kern.hibernatefile'
    KernHibernatefile,
    /// Key for 'kern.hibernategraphicsready'
    KernHibernategraphicsready,
    /// Key for 'kern.hibernatehidready'
    KernHibernatehidready,
    /// Key for 'kern.hibernatelockscreenready'
    KernHibernatelockscreenready,
    /// Key for 'kern.hibernatemode'
    KernHibernatemode,
    /// Key for 'kern.hibernatewakenotification'
    KernHibernatewakenotification,
    /// Key for 'kern.hostid'
    KernHostid,
    /// Key for 'kern.hostname'
    KernHostname,
    /// Key for 'kern.hv.vmx_mitigations'
    KernHvVmx_mitigations,
    /// Key for 'kern.hv.vmx_supported_mitigations'
    KernHvVmx_supported_mitigations,
    /// Key for 'kern.hv_support'
    KernHv_support,
    /// Key for 'kern.interrupt_timer_coalescing_enabled'
    KernInterrupt_timer_coalescing_enabled,
    /// Key for 'kern.iokittest'
    KernIokittest,
    /// Key for 'kern.iossupportversion'
    KernIossupportversion,
    /// Key for 'kern.ipc.extbkidlercvhiwat'
    KernIpcExtbkidlercvhiwat,
    /// Key for 'kern.ipc.extbkidletime'
    KernIpcExtbkidletime,
    /// Key for 'kern.ipc.io_policy.log'
    KernIpcIo_policyLog,
    /// Key for 'kern.ipc.io_policy.uuid'
    KernIpcIo_policyUuid,
    /// Key for 'kern.ipc.maxextbkidleperproc'
    KernIpcMaxextbkidleperproc,
    /// Key for 'kern.ipc.maxrecvmsgx'
    KernIpcMaxrecvmsgx,
    /// Key for 'kern.ipc.maxsendmsgx'
    KernIpcMaxsendmsgx,
    /// Key for 'kern.ipc.maxsockbuf'
    KernIpcMaxsockbuf,
    /// Key for 'kern.ipc.mb_drain_force'
    KernIpcMb_drain_force,
    /// Key for 'kern.ipc.mb_drain_maxint'
    KernIpcMb_drain_maxint,
    /// Key for 'kern.ipc.mb_normalized'
    KernIpcMb_normalized,
    /// Key for 'kern.ipc.mb_watchdog'
    KernIpcMb_watchdog,
    /// Key for 'kern.ipc.mleak_sample_factor'
    KernIpcMleak_sample_factor,
    /// Key for 'kern.ipc.njcl'
    KernIpcNjcl,
    /// Key for 'kern.ipc.njclbytes'
    KernIpcNjclbytes,
    /// Key for 'kern.ipc.nmbclusters'
    KernIpcNmbclusters,
    /// Key for 'kern.ipc.sbmb_cnt'
    KernIpcSbmb_cnt,
    /// Key for 'kern.ipc.sbmb_cnt_floor'
    KernIpcSbmb_cnt_floor,
    /// Key for 'kern.ipc.sbmb_cnt_peak'
    KernIpcSbmb_cnt_peak,
    /// Key for 'kern.ipc.sbmb_limreached'
    KernIpcSbmb_limreached,
    /// Key for 'kern.ipc.sockbuf_waste_factor'
    KernIpcSockbuf_waste_factor,
    /// Key for 'kern.ipc.socket_debug'
    KernIpcSocket_debug,
    /// Key for 'kern.ipc.sodefunct_calls'
    KernIpcSodefunct_calls,
    /// Key for 'kern.ipc.sodefunctlog'
    KernIpcSodefunctlog,
    /// Key for 'kern.ipc.somaxconn'
    KernIpcSomaxconn,
    /// Key for 'kern.ipc.soqlencomp'
    KernIpcSoqlencomp,
    /// Key for 'kern.ipc.soqlimitcompat'
    KernIpcSoqlimitcompat,
    /// Key for 'kern.ipc.sorecvmincopy'
    KernIpcSorecvmincopy,
    /// Key for 'kern.ipc.soreserveheadroom'
    KernIpcSoreserveheadroom,
    /// Key for 'kern.ipc.sorestrictrecv'
    KernIpcSorestrictrecv,
    /// Key for 'kern.ipc.sorestrictsend'
    KernIpcSorestrictsend,
    /// Key for 'kern.ipc.sosendbigcl_ignore_capab'
    KernIpcSosendbigcl_ignore_capab,
    /// Key for 'kern.ipc.sosendjcl'
    KernIpcSosendjcl,
    /// Key for 'kern.ipc.sosendjcl_ignore_capab'
    KernIpcSosendjcl_ignore_capab,
    /// Key for 'kern.ipc.sosendminchain'
    KernIpcSosendminchain,
    /// Key for 'kern.ipc.sotcdb'
    KernIpcSotcdb,
    /// Key for 'kern.ipc.sothrottlelog'
    KernIpcSothrottlelog,
    /// Key for 'kern.ipc.throttle_best_effort'
    KernIpcThrottle_best_effort,
    /// Key for 'kern.ipc_portbt'
    KernIpc_portbt,
    /// Key for 'kern.ipc_voucher_trace_contents'
    KernIpc_voucher_trace_contents,
    /// Key for 'kern.jetsam_aging_policy'
    KernJetsam_aging_policy,
    /// Key for 'kern.job_control'
    KernJob_control,
    /// Key for 'kern.kdbg.debug'
    KernKdbgDebug,
    /// Key for 'kern.kdbg.experimental_continuous'
    KernKdbgExperimental_continuous,
    /// Key for 'kern.kdbg.oldest_time'
    KernKdbgOldest_time,
    /// Key for 'kern.kern_feature_overrides'
    KernKern_feature_overrides,
    /// Key for 'kern.kernelcacheuuid'
    KernKernelcacheuuid,
    /// Key for 'kern.maxfiles'
    KernMaxfiles,
    /// Key for 'kern.maxfilesperproc'
    KernMaxfilesperproc,
    /// Key for 'kern.maxnbuf'
    KernMaxnbuf,
    /// Key for 'kern.maxproc'
    KernMaxproc,
    /// Key for 'kern.maxprocperuid'
    KernMaxprocperuid,
    /// Key for 'kern.maxvnodes'
    KernMaxvnodes,
    /// Key for 'kern.memorystatus_apps_idle_delay_time'
    KernMemorystatus_apps_idle_delay_time,
    /// Key for 'kern.memorystatus_purge_on_critical'
    KernMemorystatus_purge_on_critical,
    /// Key for 'kern.memorystatus_purge_on_urgent'
    KernMemorystatus_purge_on_urgent,
    /// Key for 'kern.memorystatus_purge_on_warning'
    KernMemorystatus_purge_on_warning,
    /// Key for 'kern.memorystatus_sysprocs_idle_delay_time'
    KernMemorystatus_sysprocs_idle_delay_time,
    /// Key for 'kern.minimalboot'
    KernMinimalboot,
    /// Key for 'kern.monotonic.pmis'
    KernMonotonicPmis,
    /// Key for 'kern.monotonic.retrograde_updates'
    KernMonotonicRetrograde_updates,
    /// Key for 'kern.monotonic.supported'
    KernMonotonicSupported,
    /// Key for 'kern.monotonic.task_thread_counting'
    KernMonotonicTask_thread_counting,
    /// Key for 'kern.msgbuf'
    KernMsgbuf,
    /// Key for 'kern.namecache_disabled'
    KernNamecache_disabled,
    /// Key for 'kern.nbuf'
    KernNbuf,
    /// Key for 'kern.netboot'
    KernNetboot,
    /// Key for 'kern.ngroups'
    KernNgroups,
    /// Key for 'kern.nisdomainname'
    KernNisdomainname,
    /// Key for 'kern.num_files'
    KernNum_files,
    /// Key for 'kern.num_recycledvnodes'
    KernNum_recycledvnodes,
    /// Key for 'kern.num_tasks'
    KernNum_tasks,
    /// Key for 'kern.num_taskthreads'
    KernNum_taskthreads,
    /// Key for 'kern.num_threads'
    KernNum_threads,
    /// Key for 'kern.num_vnodes'
    KernNum_vnodes,
    /// Key for 'kern.osproductversion'
    KernOsproductversion,
    /// Key for 'kern.osrelease'
    KernOsrelease,
    /// Key for 'kern.osrevision'
    KernOsrevision,
    /// Key for 'kern.ostype'
    KernOstype,
    /// Key for 'kern.osversion'
    KernOsversion,
    /// Key for 'kern.pmtimeout'
    KernPmtimeout,
    /// Key for 'kern.posix1version'
    KernPosix1version,
    /// Key for 'kern.posix.sem.max'
    KernPosixSemMax,
    /// Key for 'kern.preheat_max_bytes'
    KernPreheat_max_bytes,
    /// Key for 'kern.preheat_min_bytes'
    KernPreheat_min_bytes,
    /// Key for 'kern.procname'
    KernProcname,
    /// Key for 'kern.progressmeter'
    KernProgressmeter,
    /// Key for 'kern.progressmeterenable'
    KernProgressmeterenable,
    /// Key for 'kern.pthread_mutex_default_policy'
    KernPthread_mutex_default_policy,
    /// Key for 'kern.rage_vnode'
    KernRage_vnode,
    /// Key for 'kern.safeboot'
    KernSafeboot,
    /// Key for 'kern.saved_ids'
    KernSaved_ids,
    /// Key for 'kern.sched'
    KernSched,
    /// Key for 'kern.sched_allow_NO_SMT_threads'
    KernSched_allow_NO_SMT_threads,
    /// Key for 'kern.sched_enable_smt'
    KernSched_enable_smt,
    /// Key for 'kern.secure_kernel'
    KernSecure_kernel,
    /// Key for 'kern.securelevel'
    KernSecurelevel,
    /// Key for 'kern.shreg_private'
    KernShreg_private,
    /// Key for 'kern.singleuser'
    KernSingleuser,
    /// Key for 'kern.skywalk.flowswitch.awdl0.ipfm.frag_count'
    KernSkywalkFlowswitchAwdl0IpfmFrag_count,
    /// Key for 'kern.skywalk.flowswitch.awdl0.ipfm.frag_limit'
    KernSkywalkFlowswitchAwdl0IpfmFrag_limit,
    /// Key for 'kern.skywalk.flowswitch.awdl0.ipfm.queue_count'
    KernSkywalkFlowswitchAwdl0IpfmQueue_count,
    /// Key for 'kern.skywalk.flowswitch.awdl0.ipfm.queue_limit'
    KernSkywalkFlowswitchAwdl0IpfmQueue_limit,
    /// Key for 'kern.skywalk.flowswitch.en0.ipfm.frag_count'
    KernSkywalkFlowswitchEn0IpfmFrag_count,
    /// Key for 'kern.skywalk.flowswitch.en0.ipfm.frag_limit'
    KernSkywalkFlowswitchEn0IpfmFrag_limit,
    /// Key for 'kern.skywalk.flowswitch.en0.ipfm.queue_count'
    KernSkywalkFlowswitchEn0IpfmQueue_count,
    /// Key for 'kern.skywalk.flowswitch.en0.ipfm.queue_limit'
    KernSkywalkFlowswitchEn0IpfmQueue_limit,
    /// Key for 'kern.skywalk.flowswitch.en1.ipfm.frag_count'
    KernSkywalkFlowswitchEn1IpfmFrag_count,
    /// Key for 'kern.skywalk.flowswitch.en1.ipfm.frag_limit'
    KernSkywalkFlowswitchEn1IpfmFrag_limit,
    /// Key for 'kern.skywalk.flowswitch.en1.ipfm.queue_count'
    KernSkywalkFlowswitchEn1IpfmQueue_count,
    /// Key for 'kern.skywalk.flowswitch.en1.ipfm.queue_limit'
    KernSkywalkFlowswitchEn1IpfmQueue_limit,
    /// Key for 'kern.skywalk.flowswitch.en2.ipfm.frag_count'
    KernSkywalkFlowswitchEn2IpfmFrag_count,
    /// Key for 'kern.skywalk.flowswitch.en2.ipfm.frag_limit'
    KernSkywalkFlowswitchEn2IpfmFrag_limit,
    /// Key for 'kern.skywalk.flowswitch.en2.ipfm.queue_count'
    KernSkywalkFlowswitchEn2IpfmQueue_count,
    /// Key for 'kern.skywalk.flowswitch.en2.ipfm.queue_limit'
    KernSkywalkFlowswitchEn2IpfmQueue_limit,
    /// Key for 'kern.skywalk.flowswitch.flow_route_expire'
    KernSkywalkFlowswitchFlow_route_expire,
    /// Key for 'kern.skywalk.flowswitch.ip_reass'
    KernSkywalkFlowswitchIp_reass,
    /// Key for 'kern.skywalk.flowswitch.ipfm_frag_ttl'
    KernSkywalkFlowswitchIpfm_frag_ttl,
    /// Key for 'kern.skywalk.flowswitch.ipfm_timeout_tcall_ival'
    KernSkywalkFlowswitchIpfm_timeout_tcall_ival,
    /// Key for 'kern.skywalk.netif.default_drop'
    KernSkywalkNetifDefault_drop,
    /// Key for 'kern.skywalk.ring_stat_enable'
    KernSkywalkRing_stat_enable,
    /// Key for 'kern.sleep_abs_time'
    KernSleep_abs_time,
    /// Key for 'kern.sleeptime'
    KernSleeptime,
    /// Key for 'kern.slide'
    KernSlide,
    /// Key for 'kern.speculative_prefetch_max'
    KernSpeculative_prefetch_max,
    /// Key for 'kern.speculative_prefetch_max_iosize'
    KernSpeculative_prefetch_max_iosize,
    /// Key for 'kern.speculative_reads_disabled'
    KernSpeculative_reads_disabled,
    /// Key for 'kern.stack_depth_max'
    KernStack_depth_max,
    /// Key for 'kern.stack_size'
    KernStack_size,
    /// Key for 'kern.sugid_coredump'
    KernSugid_coredump,
    /// Key for 'kern.sugid_scripts'
    KernSugid_scripts,
    /// Key for 'kern.sysv.semmni'
    KernSysvSemmni,
    /// Key for 'kern.sysv.semmns'
    KernSysvSemmns,
    /// Key for 'kern.sysv.semmnu'
    KernSysvSemmnu,
    /// Key for 'kern.sysv.semmsl'
    KernSysvSemmsl,
    /// Key for 'kern.sysv.semume'
    KernSysvSemume,
    /// Key for 'kern.sysv.shmall'
    KernSysvShmall,
    /// Key for 'kern.sysv.shmmax'
    KernSysvShmmax,
    /// Key for 'kern.sysv.shmmin'
    KernSysvShmmin,
    /// Key for 'kern.sysv.shmmni'
    KernSysvShmmni,
    /// Key for 'kern.sysv.shmseg'
    KernSysvShmseg,
    /// Key for 'kern.task_exc_guard_default'
    KernTask_exc_guard_default,
    /// Key for 'kern.tfp.policy'
    KernTfpPolicy,
    /// Key for 'kern.thread_groups_supported'
    KernThread_groups_supported,
    /// Key for 'kern.threadname'
    KernThreadname,
    /// Key for 'kern.timer.coalescing_enabled'
    KernTimerCoalescing_enabled,
    /// Key for 'kern.timer.deadline_tracking_bin_1'
    KernTimerDeadline_tracking_bin_1,
    /// Key for 'kern.timer.deadline_tracking_bin_2'
    KernTimerDeadline_tracking_bin_2,
    /// Key for 'kern.timer.longterm.qlen'
    KernTimerLongtermQlen,
    /// Key for 'kern.timer.longterm.scan_interval'
    KernTimerLongtermScan_interval,
    /// Key for 'kern.timer.longterm.scan_limit'
    KernTimerLongtermScan_limit,
    /// Key for 'kern.timer.longterm.scan_pauses'
    KernTimerLongtermScan_pauses,
    /// Key for 'kern.timer.longterm.threshold'
    KernTimerLongtermThreshold,
    /// Key for 'kern.timer_coalesce_bg_ns_max'
    KernTimer_coalesce_bg_ns_max,
    /// Key for 'kern.timer_coalesce_bg_scale'
    KernTimer_coalesce_bg_scale,
    /// Key for 'kern.timer_coalesce_fp_ns_max'
    KernTimer_coalesce_fp_ns_max,
    /// Key for 'kern.timer_coalesce_fp_scale'
    KernTimer_coalesce_fp_scale,
    /// Key for 'kern.timer_coalesce_idle_entry_hard_deadline_max'
    KernTimer_coalesce_idle_entry_hard_deadline_max,
    /// Key for 'kern.timer_coalesce_kt_ns_max'
    KernTimer_coalesce_kt_ns_max,
    /// Key for 'kern.timer_coalesce_kt_scale'
    KernTimer_coalesce_kt_scale,
    /// Key for 'kern.timer_coalesce_tier0_ns_max'
    KernTimer_coalesce_tier0_ns_max,
    /// Key for 'kern.timer_coalesce_tier0_scale'
    KernTimer_coalesce_tier0_scale,
    /// Key for 'kern.timer_coalesce_tier1_ns_max'
    KernTimer_coalesce_tier1_ns_max,
    /// Key for 'kern.timer_coalesce_tier1_scale'
    KernTimer_coalesce_tier1_scale,
    /// Key for 'kern.timer_coalesce_tier2_ns_max'
    KernTimer_coalesce_tier2_ns_max,
    /// Key for 'kern.timer_coalesce_tier2_scale'
    KernTimer_coalesce_tier2_scale,
    /// Key for 'kern.timer_coalesce_tier3_ns_max'
    KernTimer_coalesce_tier3_ns_max,
    /// Key for 'kern.timer_coalesce_tier3_scale'
    KernTimer_coalesce_tier3_scale,
    /// Key for 'kern.timer_coalesce_tier4_ns_max'
    KernTimer_coalesce_tier4_ns_max,
    /// Key for 'kern.timer_coalesce_tier4_scale'
    KernTimer_coalesce_tier4_scale,
    /// Key for 'kern.timer_coalesce_tier5_ns_max'
    KernTimer_coalesce_tier5_ns_max,
    /// Key for 'kern.timer_coalesce_tier5_scale'
    KernTimer_coalesce_tier5_scale,
    /// Key for 'kern.timer_coalesce_ts_ns_max'
    KernTimer_coalesce_ts_ns_max,
    /// Key for 'kern.timer_coalesce_ts_scale'
    KernTimer_coalesce_ts_scale,
    /// Key for 'kern.timer_resort_threshold_ns'
    KernTimer_resort_threshold_ns,
    /// Key for 'kern.tty.ptmx_max'
    KernTtyPtmx_max,
    /// Key for 'kern.ulock_adaptive_spin_usecs'
    KernUlock_adaptive_spin_usecs,
    /// Key for 'kern.useractive_abs_time'
    KernUseractive_abs_time,
    /// Key for 'kern.userinactive_abs_time'
    KernUserinactive_abs_time,
    /// Key for 'kern.usrstack'
    KernUsrstack,
    /// Key for 'kern.usrstack64'
    KernUsrstack64,
    /// Key for 'kern.uuid'
    KernUuid,
    /// Key for 'kern.version'
    KernVersion,
    /// Key for 'kern.vfsnspace'
    KernVfsnspace,
    /// Key for 'kern.vm_max_batch'
    KernVm_max_batch,
    /// Key for 'kern.vm_max_delayed_work_limit'
    KernVm_max_delayed_work_limit,
    /// Key for 'kern.vm_page_free_min'
    KernVm_page_free_min,
    /// Key for 'kern.vm_page_free_reserved'
    KernVm_page_free_reserved,
    /// Key for 'kern.vm_page_free_target'
    KernVm_page_free_target,
    /// Key for 'kern.vm_page_speculative_percentage'
    KernVm_page_speculative_percentage,
    /// Key for 'kern.vm_page_speculative_q_age_ms'
    KernVm_page_speculative_q_age_ms,
    /// Key for 'kern.wake_abs_time'
    KernWake_abs_time,
    /// Key for 'kern.wakereason'
    KernWakereason,
    /// Key for 'kern.waketime'
    KernWaketime,
    /// Key for 'kern.willshutdown'
    KernWillshutdown,
    /// Key for 'kern.wq_max_constrained_threads'
    KernWq_max_constrained_threads,
    /// Key for 'kern.wq_max_threads'
    KernWq_max_threads,
    /// Key for 'kern.wq_max_timer_interval_usecs'
    KernWq_max_timer_interval_usecs,
    /// Key for 'kern.wq_reduce_pool_window_usecs'
    KernWq_reduce_pool_window_usecs,
    /// Key for 'kern.wq_stalled_window_usecs'
    KernWq_stalled_window_usecs,
    /// Key for 'kern.zleak.active'
    KernZleakActive,
    /// Key for 'kern.zleak.global_threshold'
    KernZleakGlobal_threshold,
    /// Key for 'kern.zleak.max_zonemap_size'
    KernZleakMax_zonemap_size,
    /// Key for 'kern.zleak.zone_threshold'
    KernZleakZone_threshold,
    /// Key for 'kperf.debug_level'
    KperfDebug_level,
    /// Key for 'kperf.limits.timer_min_bg_period_ns'
    KperfLimitsTimer_min_bg_period_ns,
    /// Key for 'kperf.limits.timer_min_bg_pet_period_ns'
    KperfLimitsTimer_min_bg_pet_period_ns,
    /// Key for 'kperf.limits.timer_min_period_ns'
    KperfLimitsTimer_min_period_ns,
    /// Key for 'kperf.limits.timer_min_pet_period_ns'
    KperfLimitsTimer_min_pet_period_ns,
    /// Key for 'ktrace.background_pid'
    KtraceBackground_pid,
    /// Key for 'ktrace.configured_by'
    KtraceConfigured_by,
    /// Key for 'ktrace.owning_pid'
    KtraceOwning_pid,
    /// Key for 'ktrace.state'
    KtraceState,
    /// Key for 'machdep.cpu.address_bits.physical'
    MachdepCpuAddress_bitsPhysical,
    /// Key for 'machdep.cpu.address_bits.virtual'
    MachdepCpuAddress_bitsVirtual,
    /// Key for 'machdep.cpu.arch_perf.events'
    MachdepCpuArch_perfEvents,
    /// Key for 'machdep.cpu.arch_perf.events_number'
    MachdepCpuArch_perfEvents_number,
    /// Key for 'machdep.cpu.arch_perf.fixed_number'
    MachdepCpuArch_perfFixed_number,
    /// Key for 'machdep.cpu.arch_perf.fixed_width'
    MachdepCpuArch_perfFixed_width,
    /// Key for 'machdep.cpu.arch_perf.number'
    MachdepCpuArch_perfNumber,
    /// Key for 'machdep.cpu.arch_perf.version'
    MachdepCpuArch_perfVersion,
    /// Key for 'machdep.cpu.arch_perf.width'
    MachdepCpuArch_perfWidth,
    /// Key for 'machdep.cpu.brand'
    MachdepCpuBrand,
    /// Key for 'machdep.cpu.brand_string'
    MachdepCpuBrand_string,
    /// Key for 'machdep.cpu.cache.L2_associativity'
    MachdepCpuCacheL2_associativity,
    /// Key for 'machdep.cpu.cache.linesize'
    MachdepCpuCacheLinesize,
    /// Key for 'machdep.cpu.cache.size'
    MachdepCpuCacheSize,
    /// Key for 'machdep.cpu.core_count'
    MachdepCpuCore_count,
    /// Key for 'machdep.cpu.cores_per_package'
    MachdepCpuCores_per_package,
    /// Key for 'machdep.cpu.extfamily'
    MachdepCpuExtfamily,
    /// Key for 'machdep.cpu.extfeature_bits'
    MachdepCpuExtfeature_bits,
    /// Key for 'machdep.cpu.extfeatures'
    MachdepCpuExtfeatures,
    /// Key for 'machdep.cpu.extmodel'
    MachdepCpuExtmodel,
    /// Key for 'machdep.cpu.family'
    MachdepCpuFamily,
    /// Key for 'machdep.cpu.feature_bits'
    MachdepCpuFeature_bits,
    /// Key for 'machdep.cpu.features'
    MachdepCpuFeatures,
    /// Key for 'machdep.cpu.leaf7_feature_bits'
    MachdepCpuLeaf7_feature_bits,
    /// Key for 'machdep.cpu.leaf7_feature_bits_edx'
    MachdepCpuLeaf7_feature_bits_edx,
    /// Key for 'machdep.cpu.leaf7_features'
    MachdepCpuLeaf7_features,
    /// Key for 'machdep.cpu.logical_per_package'
    MachdepCpuLogical_per_package,
    /// Key for 'machdep.cpu.max_basic'
    MachdepCpuMax_basic,
    /// Key for 'machdep.cpu.max_ext'
    MachdepCpuMax_ext,
    /// Key for 'machdep.cpu.microcode_version'
    MachdepCpuMicrocode_version,
    /// Key for 'machdep.cpu.model'
    MachdepCpuModel,
    /// Key for 'machdep.cpu.mwait.extensions'
    MachdepCpuMwaitExtensions,
    /// Key for 'machdep.cpu.mwait.linesize_max'
    MachdepCpuMwaitLinesize_max,
    /// Key for 'machdep.cpu.mwait.linesize_min'
    MachdepCpuMwaitLinesize_min,
    /// Key for 'machdep.cpu.mwait.sub_Cstates'
    MachdepCpuMwaitSub_Cstates,
    /// Key for 'machdep.cpu.processor_flag'
    MachdepCpuProcessor_flag,
    /// Key for 'machdep.cpu.signature'
    MachdepCpuSignature,
    /// Key for 'machdep.cpu.stepping'
    MachdepCpuStepping,
    /// Key for 'machdep.cpu.thermal.ACNT_MCNT'
    MachdepCpuThermalACNT_MCNT,
    /// Key for 'machdep.cpu.thermal.core_power_limits'
    MachdepCpuThermalCore_power_limits,
    /// Key for 'machdep.cpu.thermal.dynamic_acceleration'
    MachdepCpuThermalDynamic_acceleration,
    /// Key for 'machdep.cpu.thermal.energy_policy'
    MachdepCpuThermalEnergy_policy,
    /// Key for 'machdep.cpu.thermal.fine_grain_clock_mod'
    MachdepCpuThermalFine_grain_clock_mod,
    /// Key for 'machdep.cpu.thermal.hardware_feedback'
    MachdepCpuThermalHardware_feedback,
    /// Key for 'machdep.cpu.thermal.invariant_APIC_timer'
    MachdepCpuThermalInvariant_APIC_timer,
    /// Key for 'machdep.cpu.thermal.package_thermal_intr'
    MachdepCpuThermalPackage_thermal_intr,
    /// Key for 'machdep.cpu.thermal.sensor'
    MachdepCpuThermalSensor,
    /// Key for 'machdep.cpu.thermal.thresholds'
    MachdepCpuThermalThresholds,
    /// Key for 'machdep.cpu.thread_count'
    MachdepCpuThread_count,
    /// Key for 'machdep.cpu.tlb.data.small'
    MachdepCpuTlbDataSmall,
    /// Key for 'machdep.cpu.tlb.data.small_level1'
    MachdepCpuTlbDataSmall_level1,
    /// Key for 'machdep.cpu.tlb.inst.large'
    MachdepCpuTlbInstLarge,
    /// Key for 'machdep.cpu.tsc_ccc.denominator'
    MachdepCpuTsc_cccDenominator,
    /// Key for 'machdep.cpu.tsc_ccc.numerator'
    MachdepCpuTsc_cccNumerator,
    /// Key for 'machdep.cpu.vendor'
    MachdepCpuVendor,
    /// Key for 'machdep.cpu.xsave.extended_state'
    MachdepCpuXsaveExtended_state,
    /// Key for 'machdep.cpu.xsave.extended_state1'
    MachdepCpuXsaveExtended_state1,
    /// Key for 'machdep.eager_timer_evaluation_max'
    MachdepEager_timer_evaluation_max,
    /// Key for 'machdep.eager_timer_evaluations'
    MachdepEager_timer_evaluations,
    /// Key for 'machdep.memmap.ACPINVS'
    MachdepMemmapACPINVS,
    /// Key for 'machdep.memmap.ACPIReclaim'
    MachdepMemmapACPIReclaim,
    /// Key for 'machdep.memmap.Conventional'
    MachdepMemmapConventional,
    /// Key for 'machdep.memmap.Other'
    MachdepMemmapOther,
    /// Key for 'machdep.memmap.PalCode'
    MachdepMemmapPalCode,
    /// Key for 'machdep.memmap.Reserved'
    MachdepMemmapReserved,
    /// Key for 'machdep.memmap.RuntimeServices'
    MachdepMemmapRuntimeServices,
    /// Key for 'machdep.memmap.Unusable'
    MachdepMemmapUnusable,
    /// Key for 'machdep.misc.fast_uexc_support'
    MachdepMiscFast_uexc_support,
    /// Key for 'machdep.misc.interrupt_latency_max'
    MachdepMiscInterrupt_latency_max,
    /// Key for 'machdep.misc.nmis'
    MachdepMiscNmis,
    /// Key for 'machdep.misc.panic_restart_timeout'
    MachdepMiscPanic_restart_timeout,
    /// Key for 'machdep.misc.timer_queue_trace'
    MachdepMiscTimer_queue_trace,
    /// Key for 'machdep.pmap.hashcnts'
    MachdepPmapHashcnts,
    /// Key for 'machdep.pmap.hashmax'
    MachdepPmapHashmax,
    /// Key for 'machdep.pmap.hashwalks'
    MachdepPmapHashwalks,
    /// Key for 'machdep.pmap.kern_pv_reserve'
    MachdepPmapKern_pv_reserve,
    /// Key for 'machdep.pmap.kernel_text_ps'
    MachdepPmapKernel_text_ps,
    /// Key for 'machdep.tsc.at_boot'
    MachdepTscAt_boot,
    /// Key for 'machdep.tsc.deep_idle_rebase'
    MachdepTscDeep_idle_rebase,
    /// Key for 'machdep.tsc.frequency'
    MachdepTscFrequency,
    /// Key for 'machdep.tsc.nanotime.generation'
    MachdepTscNanotimeGeneration,
    /// Key for 'machdep.tsc.nanotime.ns_base'
    MachdepTscNanotimeNs_base,
    /// Key for 'machdep.tsc.nanotime.scale'
    MachdepTscNanotimeScale,
    /// Key for 'machdep.tsc.nanotime.shift'
    MachdepTscNanotimeShift,
    /// Key for 'machdep.tsc.nanotime.tsc_base'
    MachdepTscNanotimeTsc_base,
    /// Key for 'machdep.tsc.rebase_abs_time'
    MachdepTscRebase_abs_time,
    /// Key for 'machdep.user_idle_level'
    MachdepUser_idle_level,
    /// Key for 'machdep.vectors.IPI'
    MachdepVectorsIPI,
    /// Key for 'machdep.vectors.timer'
    MachdepVectorsTimer,
    /// Key for 'machdep.x86_fp_simd_isr_uses'
    MachdepX86_fp_simd_isr_uses,
    /// Key for 'machdep.xcpm.bootplim'
    MachdepXcpmBootplim,
    /// Key for 'machdep.xcpm.bootpst'
    MachdepXcpmBootpst,
    /// Key for 'machdep.xcpm.cpu_thermal_level'
    MachdepXcpmCpu_thermal_level,
    /// Key for 'machdep.xcpm.deep_idle_count'
    MachdepXcpmDeep_idle_count,
    /// Key for 'machdep.xcpm.deep_idle_last_stats'
    MachdepXcpmDeep_idle_last_stats,
    /// Key for 'machdep.xcpm.deep_idle_log'
    MachdepXcpmDeep_idle_log,
    /// Key for 'machdep.xcpm.deep_idle_total_stats'
    MachdepXcpmDeep_idle_total_stats,
    /// Key for 'machdep.xcpm.epp_override'
    MachdepXcpmEpp_override,
    /// Key for 'machdep.xcpm.forced_idle_period'
    MachdepXcpmForced_idle_period,
    /// Key for 'machdep.xcpm.forced_idle_ratio'
    MachdepXcpmForced_idle_ratio,
    /// Key for 'machdep.xcpm.gpu_thermal_level'
    MachdepXcpmGpu_thermal_level,
    /// Key for 'machdep.xcpm.hard_plimit_max_100mhz_ratio'
    MachdepXcpmHard_plimit_max_100mhz_ratio,
    /// Key for 'machdep.xcpm.hard_plimit_min_100mhz_ratio'
    MachdepXcpmHard_plimit_min_100mhz_ratio,
    /// Key for 'machdep.xcpm.io_control_disengages'
    MachdepXcpmIo_control_disengages,
    /// Key for 'machdep.xcpm.io_control_engages'
    MachdepXcpmIo_control_engages,
    /// Key for 'machdep.xcpm.io_cst_control_enabled'
    MachdepXcpmIo_cst_control_enabled,
    /// Key for 'machdep.xcpm.io_epp_boost_enabled'
    MachdepXcpmIo_epp_boost_enabled,
    /// Key for 'machdep.xcpm.io_filtered_reads'
    MachdepXcpmIo_filtered_reads,
    /// Key for 'machdep.xcpm.io_thermal_level'
    MachdepXcpmIo_thermal_level,
    /// Key for 'machdep.xcpm.maxbusdelay'
    MachdepXcpmMaxbusdelay,
    /// Key for 'machdep.xcpm.maxintdelay'
    MachdepXcpmMaxintdelay,
    /// Key for 'machdep.xcpm.mbd_applications'
    MachdepXcpmMbd_applications,
    /// Key for 'machdep.xcpm.mbd_mode'
    MachdepXcpmMbd_mode,
    /// Key for 'machdep.xcpm.mbd_relaxations'
    MachdepXcpmMbd_relaxations,
    /// Key for 'machdep.xcpm.mid_applications'
    MachdepXcpmMid_applications,
    /// Key for 'machdep.xcpm.mid_cst_control_limit'
    MachdepXcpmMid_cst_control_limit,
    /// Key for 'machdep.xcpm.mid_mode'
    MachdepXcpmMid_mode,
    /// Key for 'machdep.xcpm.mid_mode_active'
    MachdepXcpmMid_mode_active,
    /// Key for 'machdep.xcpm.mid_relaxations'
    MachdepXcpmMid_relaxations,
    /// Key for 'machdep.xcpm.mode'
    MachdepXcpmMode,
    /// Key for 'machdep.xcpm.pcps_mode'
    MachdepXcpmPcps_mode,
    /// Key for 'machdep.xcpm.pcps_rt_override_mode'
    MachdepXcpmPcps_rt_override_mode,
    /// Key for 'machdep.xcpm.pcps_rt_override_ns'
    MachdepXcpmPcps_rt_override_ns,
    /// Key for 'machdep.xcpm.perf_hints'
    MachdepXcpmPerf_hints,
    /// Key for 'machdep.xcpm.power_source'
    MachdepXcpmPower_source,
    /// Key for 'machdep.xcpm.qos_txfr'
    MachdepXcpmQos_txfr,
    /// Key for 'machdep.xcpm.ratio_change_ratelimit_ns'
    MachdepXcpmRatio_change_ratelimit_ns,
    /// Key for 'machdep.xcpm.ratio_changes_total'
    MachdepXcpmRatio_changes_total,
    /// Key for 'machdep.xcpm.ring_boost_enabled'
    MachdepXcpmRing_boost_enabled,
    /// Key for 'machdep.xcpm.soft_plimit_max_100mhz_ratio'
    MachdepXcpmSoft_plimit_max_100mhz_ratio,
    /// Key for 'machdep.xcpm.soft_plimit_min_100mhz_ratio'
    MachdepXcpmSoft_plimit_min_100mhz_ratio,
    /// Key for 'machdep.xcpm.tuib_enabled'
    MachdepXcpmTuib_enabled,
    /// Key for 'machdep.xcpm.tuib_ns'
    MachdepXcpmTuib_ns,
    /// Key for 'machdep.xcpm.tuib_plimit_max_100mhz_ratio'
    MachdepXcpmTuib_plimit_max_100mhz_ratio,
    /// Key for 'machdep.xcpm.tuib_plimit_min_100mhz_ratio'
    MachdepXcpmTuib_plimit_min_100mhz_ratio,
    /// Key for 'machdep.xcpm.vectors_loaded_count'
    MachdepXcpmVectors_loaded_count,
    /// Key for 'net.alf.defaultaction'
    NetAlfDefaultaction,
    /// Key for 'net.alf.loglevel'
    NetAlfLoglevel,
    /// Key for 'net.alf.mqcount'
    NetAlfMqcount,
    /// Key for 'net.alf.perm'
    NetAlfPerm,
    /// Key for 'net.cfil.active_count'
    NetCfilActive_count,
    /// Key for 'net.cfil.close_wait_timeout'
    NetCfilClose_wait_timeout,
    /// Key for 'net.cfil.debug'
    NetCfilDebug,
    /// Key for 'net.cfil.log'
    NetCfilLog,
    /// Key for 'net.cfil.sbtrim'
    NetCfilSbtrim,
    /// Key for 'net.cfil.sock_attached_count'
    NetCfilSock_attached_count,
    /// Key for 'net.classq.sfb.allocation'
    NetClassqSfbAllocation,
    /// Key for 'net.classq.sfb.decrement'
    NetClassqSfbDecrement,
    /// Key for 'net.classq.sfb.hinterval'
    NetClassqSfbHinterval,
    /// Key for 'net.classq.sfb.holdtime'
    NetClassqSfbHoldtime,
    /// Key for 'net.classq.sfb.increment'
    NetClassqSfbIncrement,
    /// Key for 'net.classq.sfb.pboxtime'
    NetClassqSfbPboxtime,
    /// Key for 'net.classq.sfb.ratelimit'
    NetClassqSfbRatelimit,
    /// Key for 'net.classq.target_qdelay'
    NetClassqTarget_qdelay,
    /// Key for 'net.classq.update_interval'
    NetClassqUpdate_interval,
    /// Key for 'net.classq.verbose'
    NetClassqVerbose,
    /// Key for 'net.inet6.icmp6.errppslimit'
    NetInet6Icmp6Errppslimit,
    /// Key for 'net.inet6.icmp6.nd6_accept_6to4'
    NetInet6Icmp6Nd6_accept_6to4,
    /// Key for 'net.inet6.icmp6.nd6_debug'
    NetInet6Icmp6Nd6_debug,
    /// Key for 'net.inet6.icmp6.nd6_delay'
    NetInet6Icmp6Nd6_delay,
    /// Key for 'net.inet6.icmp6.nd6_llreach_base'
    NetInet6Icmp6Nd6_llreach_base,
    /// Key for 'net.inet6.icmp6.nd6_maxproxiedsol'
    NetInet6Icmp6Nd6_maxproxiedsol,
    /// Key for 'net.inet6.icmp6.nd6_maxsolstgt'
    NetInet6Icmp6Nd6_maxsolstgt,
    /// Key for 'net.inet6.icmp6.nd6_mmaxtries'
    NetInet6Icmp6Nd6_mmaxtries,
    /// Key for 'net.inet6.icmp6.nd6_onlink_ns_rfc4861'
    NetInet6Icmp6Nd6_onlink_ns_rfc4861,
    /// Key for 'net.inet6.icmp6.nd6_optimistic_dad'
    NetInet6Icmp6Nd6_optimistic_dad,
    /// Key for 'net.inet6.icmp6.nd6_prune'
    NetInet6Icmp6Nd6_prune,
    /// Key for 'net.inet6.icmp6.nd6_prune_lazy'
    NetInet6Icmp6Nd6_prune_lazy,
    /// Key for 'net.inet6.icmp6.nd6_umaxtries'
    NetInet6Icmp6Nd6_umaxtries,
    /// Key for 'net.inet6.icmp6.nd6_useloopback'
    NetInet6Icmp6Nd6_useloopback,
    /// Key for 'net.inet6.icmp6.nodeinfo'
    NetInet6Icmp6Nodeinfo,
    /// Key for 'net.inet6.icmp6.prproxy_cnt'
    NetInet6Icmp6Prproxy_cnt,
    /// Key for 'net.inet6.icmp6.rappslimit'
    NetInet6Icmp6Rappslimit,
    /// Key for 'net.inet6.icmp6.rediraccept'
    NetInet6Icmp6Rediraccept,
    /// Key for 'net.inet6.icmp6.redirtimeout'
    NetInet6Icmp6Redirtimeout,
    /// Key for 'net.inet6.ip6.accept_rtadv'
    NetInet6Ip6Accept_rtadv,
    /// Key for 'net.inet6.ip6.adj_clear_hwcksum'
    NetInet6Ip6Adj_clear_hwcksum,
    /// Key for 'net.inet6.ip6.adj_partial_sum'
    NetInet6Ip6Adj_partial_sum,
    /// Key for 'net.inet6.ip6.auto_flowlabel'
    NetInet6Ip6Auto_flowlabel,
    /// Key for 'net.inet6.ip6.auto_linklocal'
    NetInet6Ip6Auto_linklocal,
    /// Key for 'net.inet6.ip6.check_interface'
    NetInet6Ip6Check_interface,
    /// Key for 'net.inet6.ip6.checkinterface_debug'
    NetInet6Ip6Checkinterface_debug,
    /// Key for 'net.inet6.ip6.clat_debug'
    NetInet6Ip6Clat_debug,
    /// Key for 'net.inet6.ip6.dad_count'
    NetInet6Ip6Dad_count,
    /// Key for 'net.inet6.ip6.dad_enhanced'
    NetInet6Ip6Dad_enhanced,
    /// Key for 'net.inet6.ip6.defmcasthlim'
    NetInet6Ip6Defmcasthlim,
    /// Key for 'net.inet6.ip6.forwarding'
    NetInet6Ip6Forwarding,
    /// Key for 'net.inet6.ip6.fragpackets'
    NetInet6Ip6Fragpackets,
    /// Key for 'net.inet6.ip6.gifhlim'
    NetInet6Ip6Gifhlim,
    /// Key for 'net.inet6.ip6.hdrnestlimit'
    NetInet6Ip6Hdrnestlimit,
    /// Key for 'net.inet6.ip6.hlim'
    NetInet6Ip6Hlim,
    /// Key for 'net.inet6.ip6.input_perf'
    NetInet6Ip6Input_perf,
    /// Key for 'net.inet6.ip6.input_perf_bins'
    NetInet6Ip6Input_perf_bins,
    /// Key for 'net.inet6.ip6.kame_version'
    NetInet6Ip6Kame_version,
    /// Key for 'net.inet6.ip6.keepfaith'
    NetInet6Ip6Keepfaith,
    /// Key for 'net.inet6.ip6.log_interval'
    NetInet6Ip6Log_interval,
    /// Key for 'net.inet6.ip6.maxchainsent'
    NetInet6Ip6Maxchainsent,
    /// Key for 'net.inet6.ip6.maxdynroutes'
    NetInet6Ip6Maxdynroutes,
    /// Key for 'net.inet6.ip6.maxfragpackets'
    NetInet6Ip6Maxfragpackets,
    /// Key for 'net.inet6.ip6.maxfrags'
    NetInet6Ip6Maxfrags,
    /// Key for 'net.inet6.ip6.maxifdefrouters'
    NetInet6Ip6Maxifdefrouters,
    /// Key for 'net.inet6.ip6.maxifprefixes'
    NetInet6Ip6Maxifprefixes,
    /// Key for 'net.inet6.ip6.mcast.loop'
    NetInet6Ip6McastLoop,
    /// Key for 'net.inet6.ip6.mcast.maxgrpsrc'
    NetInet6Ip6McastMaxgrpsrc,
    /// Key for 'net.inet6.ip6.mcast.maxsocksrc'
    NetInet6Ip6McastMaxsocksrc,
    /// Key for 'net.inet6.ip6.mcast_pmtu'
    NetInet6Ip6Mcast_pmtu,
    /// Key for 'net.inet6.ip6.neighborgcthresh'
    NetInet6Ip6Neighborgcthresh,
    /// Key for 'net.inet6.ip6.only_allow_rfc4193_prefixes'
    NetInet6Ip6Only_allow_rfc4193_prefixes,
    /// Key for 'net.inet6.ip6.output_perf'
    NetInet6Ip6Output_perf,
    /// Key for 'net.inet6.ip6.output_perf_bins'
    NetInet6Ip6Output_perf_bins,
    /// Key for 'net.inet6.ip6.prefer_tempaddr'
    NetInet6Ip6Prefer_tempaddr,
    /// Key for 'net.inet6.ip6.redirect'
    NetInet6Ip6Redirect,
    /// Key for 'net.inet6.ip6.rr_prune'
    NetInet6Ip6Rr_prune,
    /// Key for 'net.inet6.ip6.rtexpire'
    NetInet6Ip6Rtexpire,
    /// Key for 'net.inet6.ip6.rtmaxcache'
    NetInet6Ip6Rtmaxcache,
    /// Key for 'net.inet6.ip6.rtminexpire'
    NetInet6Ip6Rtminexpire,
    /// Key for 'net.inet6.ip6.select_src_expensive_secondary_if'
    NetInet6Ip6Select_src_expensive_secondary_if,
    /// Key for 'net.inet6.ip6.select_src_strong_end'
    NetInet6Ip6Select_src_strong_end,
    /// Key for 'net.inet6.ip6.select_srcaddr_debug'
    NetInet6Ip6Select_srcaddr_debug,
    /// Key for 'net.inet6.ip6.select_srcif_debug'
    NetInet6Ip6Select_srcif_debug,
    /// Key for 'net.inet6.ip6.temppltime'
    NetInet6Ip6Temppltime,
    /// Key for 'net.inet6.ip6.tempvltime'
    NetInet6Ip6Tempvltime,
    /// Key for 'net.inet6.ip6.use_defaultzone'
    NetInet6Ip6Use_defaultzone,
    /// Key for 'net.inet6.ip6.use_deprecated'
    NetInet6Ip6Use_deprecated,
    /// Key for 'net.inet6.ip6.use_tempaddr'
    NetInet6Ip6Use_tempaddr,
    /// Key for 'net.inet6.ip6.v6only'
    NetInet6Ip6V6only,
    /// Key for 'net.inet6.ipsec6.ah_net_deflev'
    NetInet6Ipsec6Ah_net_deflev,
    /// Key for 'net.inet6.ipsec6.ah_trans_deflev'
    NetInet6Ipsec6Ah_trans_deflev,
    /// Key for 'net.inet6.ipsec6.debug'
    NetInet6Ipsec6Debug,
    /// Key for 'net.inet6.ipsec6.def_policy'
    NetInet6Ipsec6Def_policy,
    /// Key for 'net.inet6.ipsec6.ecn'
    NetInet6Ipsec6Ecn,
    /// Key for 'net.inet6.ipsec6.esp_net_deflev'
    NetInet6Ipsec6Esp_net_deflev,
    /// Key for 'net.inet6.ipsec6.esp_randpad'
    NetInet6Ipsec6Esp_randpad,
    /// Key for 'net.inet6.ipsec6.esp_trans_deflev'
    NetInet6Ipsec6Esp_trans_deflev,
    /// Key for 'net.inet6.mld.debug'
    NetInet6MldDebug,
    /// Key for 'net.inet6.mld.gsrdelay'
    NetInet6MldGsrdelay,
    /// Key for 'net.inet6.mld.use_allow'
    NetInet6MldUse_allow,
    /// Key for 'net.inet6.mld.v1enable'
    NetInet6MldV1enable,
    /// Key for 'net.inet6.mld.v2enable'
    NetInet6MldV2enable,
    /// Key for 'net.inet6.send.opmode'
    NetInet6SendOpmode,
    /// Key for 'net.inet6.send.opstate'
    NetInet6SendOpstate,
    /// Key for 'net.inet.icmp.bmcastecho'
    NetInetIcmpBmcastecho,
    /// Key for 'net.inet.icmp.drop_redirect'
    NetInetIcmpDrop_redirect,
    /// Key for 'net.inet.icmp.icmplim'
    NetInetIcmpIcmplim,
    /// Key for 'net.inet.icmp.log_redirect'
    NetInetIcmpLog_redirect,
    /// Key for 'net.inet.icmp.maskrepl'
    NetInetIcmpMaskrepl,
    /// Key for 'net.inet.icmp.timestamp'
    NetInetIcmpTimestamp,
    /// Key for 'net.inet.igmp.debug'
    NetInetIgmpDebug,
    /// Key for 'net.inet.igmp.default_version'
    NetInetIgmpDefault_version,
    /// Key for 'net.inet.igmp.gsrdelay'
    NetInetIgmpGsrdelay,
    /// Key for 'net.inet.igmp.legacysupp'
    NetInetIgmpLegacysupp,
    /// Key for 'net.inet.igmp.recvifkludge'
    NetInetIgmpRecvifkludge,
    /// Key for 'net.inet.igmp.sendlocal'
    NetInetIgmpSendlocal,
    /// Key for 'net.inet.igmp.sendra'
    NetInetIgmpSendra,
    /// Key for 'net.inet.igmp.v1enable'
    NetInetIgmpV1enable,
    /// Key for 'net.inet.igmp.v2enable'
    NetInetIgmpV2enable,
    /// Key for 'net.inet.ip.accept_sourceroute'
    NetInetIpAccept_sourceroute,
    /// Key for 'net.inet.ip.adj_clear_hwcksum'
    NetInetIpAdj_clear_hwcksum,
    /// Key for 'net.inet.ip.adj_partial_sum'
    NetInetIpAdj_partial_sum,
    /// Key for 'net.inet.ip.check_interface'
    NetInetIpCheck_interface,
    /// Key for 'net.inet.ip.checkinterface_debug'
    NetInetIpCheckinterface_debug,
    /// Key for 'net.inet.ip.dummynet.curr_time'
    NetInetIpDummynetCurr_time,
    /// Key for 'net.inet.ip.dummynet.debug'
    NetInetIpDummynetDebug,
    /// Key for 'net.inet.ip.dummynet.expire'
    NetInetIpDummynetExpire,
    /// Key for 'net.inet.ip.dummynet.extract_heap'
    NetInetIpDummynetExtract_heap,
    /// Key for 'net.inet.ip.dummynet.hash_size'
    NetInetIpDummynetHash_size,
    /// Key for 'net.inet.ip.dummynet.max_chain_len'
    NetInetIpDummynetMax_chain_len,
    /// Key for 'net.inet.ip.dummynet.ready_heap'
    NetInetIpDummynetReady_heap,
    /// Key for 'net.inet.ip.dummynet.red_avg_pkt_size'
    NetInetIpDummynetRed_avg_pkt_size,
    /// Key for 'net.inet.ip.dummynet.red_lookup_depth'
    NetInetIpDummynetRed_lookup_depth,
    /// Key for 'net.inet.ip.dummynet.red_max_pkt_size'
    NetInetIpDummynetRed_max_pkt_size,
    /// Key for 'net.inet.ip.dummynet.search_steps'
    NetInetIpDummynetSearch_steps,
    /// Key for 'net.inet.ip.dummynet.searches'
    NetInetIpDummynetSearches,
    /// Key for 'net.inet.ip.forwarding'
    NetInetIpForwarding,
    /// Key for 'net.inet.ip.fragpackets'
    NetInetIpFragpackets,
    /// Key for 'net.inet.ip.gifttl'
    NetInetIpGifttl,
    /// Key for 'net.inet.ip.linklocal.in.allowbadttl'
    NetInetIpLinklocalInAllowbadttl,
    /// Key for 'net.inet.ip.maxchainsent'
    NetInetIpMaxchainsent,
    /// Key for 'net.inet.ip.maxfragpackets'
    NetInetIpMaxfragpackets,
    /// Key for 'net.inet.ip.maxfragsperpacket'
    NetInetIpMaxfragsperpacket,
    /// Key for 'net.inet.ip.mcast.loop'
    NetInetIpMcastLoop,
    /// Key for 'net.inet.ip.mcast.maxgrpsrc'
    NetInetIpMcastMaxgrpsrc,
    /// Key for 'net.inet.ip.mcast.maxsocksrc'
    NetInetIpMcastMaxsocksrc,
    /// Key for 'net.inet.ip.output_perf'
    NetInetIpOutput_perf,
    /// Key for 'net.inet.ip.output_perf_bins'
    NetInetIpOutput_perf_bins,
    /// Key for 'net.inet.ip.portrange.first'
    NetInetIpPortrangeFirst,
    /// Key for 'net.inet.ip.portrange.hifirst'
    NetInetIpPortrangeHifirst,
    /// Key for 'net.inet.ip.portrange.hilast'
    NetInetIpPortrangeHilast,
    /// Key for 'net.inet.ip.portrange.last'
    NetInetIpPortrangeLast,
    /// Key for 'net.inet.ip.portrange.lowfirst'
    NetInetIpPortrangeLowfirst,
    /// Key for 'net.inet.ip.portrange.lowlast'
    NetInetIpPortrangeLowlast,
    /// Key for 'net.inet.ip.random_id'
    NetInetIpRandom_id,
    /// Key for 'net.inet.ip.random_id_collisions'
    NetInetIpRandom_id_collisions,
    /// Key for 'net.inet.ip.random_id_statistics'
    NetInetIpRandom_id_statistics,
    /// Key for 'net.inet.ip.random_id_total'
    NetInetIpRandom_id_total,
    /// Key for 'net.inet.ip.redirect'
    NetInetIpRedirect,
    /// Key for 'net.inet.ip.rfc6864'
    NetInetIpRfc6864,
    /// Key for 'net.inet.ip.rtexpire'
    NetInetIpRtexpire,
    /// Key for 'net.inet.ip.rtmaxcache'
    NetInetIpRtmaxcache,
    /// Key for 'net.inet.ip.rtminexpire'
    NetInetIpRtminexpire,
    /// Key for 'net.inet.ip.rx_chaining'
    NetInetIpRx_chaining,
    /// Key for 'net.inet.ip.rx_chainsz'
    NetInetIpRx_chainsz,
    /// Key for 'net.inet.ip.select_srcif_debug'
    NetInetIpSelect_srcif_debug,
    /// Key for 'net.inet.ip.sendsourcequench'
    NetInetIpSendsourcequench,
    /// Key for 'net.inet.ip.sourceroute'
    NetInetIpSourceroute,
    /// Key for 'net.inet.ip.subnets_are_local'
    NetInetIpSubnets_are_local,
    /// Key for 'net.inet.ip.ttl'
    NetInetIpTtl,
    /// Key for 'net.inet.ipsec.ah_cleartos'
    NetInetIpsecAh_cleartos,
    /// Key for 'net.inet.ipsec.ah_net_deflev'
    NetInetIpsecAh_net_deflev,
    /// Key for 'net.inet.ipsec.ah_offsetmask'
    NetInetIpsecAh_offsetmask,
    /// Key for 'net.inet.ipsec.ah_trans_deflev'
    NetInetIpsecAh_trans_deflev,
    /// Key for 'net.inet.ipsec.bypass'
    NetInetIpsecBypass,
    /// Key for 'net.inet.ipsec.debug'
    NetInetIpsecDebug,
    /// Key for 'net.inet.ipsec.def_policy'
    NetInetIpsecDef_policy,
    /// Key for 'net.inet.ipsec.dfbit'
    NetInetIpsecDfbit,
    /// Key for 'net.inet.ipsec.ecn'
    NetInetIpsecEcn,
    /// Key for 'net.inet.ipsec.esp_net_deflev'
    NetInetIpsecEsp_net_deflev,
    /// Key for 'net.inet.ipsec.esp_port'
    NetInetIpsecEsp_port,
    /// Key for 'net.inet.ipsec.esp_randpad'
    NetInetIpsecEsp_randpad,
    /// Key for 'net.inet.ipsec.esp_trans_deflev'
    NetInetIpsecEsp_trans_deflev,
    /// Key for 'net.inet.log_restricted'
    NetInetLog_restricted,
    /// Key for 'net.inet.mptcp.allow_aggregate'
    NetInetMptcpAllow_aggregate,
    /// Key for 'net.inet.mptcp.alternate_port'
    NetInetMptcpAlternate_port,
    /// Key for 'net.inet.mptcp.dbg_area'
    NetInetMptcpDbg_area,
    /// Key for 'net.inet.mptcp.dbg_level'
    NetInetMptcpDbg_level,
    /// Key for 'net.inet.mptcp.dss_csum'
    NetInetMptcpDss_csum,
    /// Key for 'net.inet.mptcp.enable'
    NetInetMptcpEnable,
    /// Key for 'net.inet.mptcp.expected_progress_headstart'
    NetInetMptcpExpected_progress_headstart,
    /// Key for 'net.inet.mptcp.fail'
    NetInetMptcpFail,
    /// Key for 'net.inet.mptcp.keepalive'
    NetInetMptcpKeepalive,
    /// Key for 'net.inet.mptcp.mptcp_cap_retr'
    NetInetMptcpMptcp_cap_retr,
    /// Key for 'net.inet.mptcp.nrto'
    NetInetMptcpNrto,
    /// Key for 'net.inet.mptcp.pcbcount'
    NetInetMptcpPcbcount,
    /// Key for 'net.inet.mptcp.probecnt'
    NetInetMptcpProbecnt,
    /// Key for 'net.inet.mptcp.probeto'
    NetInetMptcpProbeto,
    /// Key for 'net.inet.mptcp.rto'
    NetInetMptcpRto,
    /// Key for 'net.inet.mptcp.rto_thresh'
    NetInetMptcpRto_thresh,
    /// Key for 'net.inet.mptcp.rtthist_thresh'
    NetInetMptcpRtthist_thresh,
    /// Key for 'net.inet.mptcp.tw'
    NetInetMptcpTw,
    /// Key for 'net.inet.mptcp.userto'
    NetInetMptcpUserto,
    /// Key for 'net.inet.raw.maxdgram'
    NetInetRawMaxdgram,
    /// Key for 'net.inet.raw.pcbcount'
    NetInetRawPcbcount,
    /// Key for 'net.inet.raw.recvspace'
    NetInetRawRecvspace,
    /// Key for 'net.inet.tcp.acc_iaj_react_limit'
    NetInetTcpAcc_iaj_react_limit,
    /// Key for 'net.inet.tcp.ack_prioritize'
    NetInetTcpAck_prioritize,
    /// Key for 'net.inet.tcp.always_keepalive'
    NetInetTcpAlways_keepalive,
    /// Key for 'net.inet.tcp.autorcvbufmax'
    NetInetTcpAutorcvbufmax,
    /// Key for 'net.inet.tcp.autosndbufinc'
    NetInetTcpAutosndbufinc,
    /// Key for 'net.inet.tcp.autosndbufmax'
    NetInetTcpAutosndbufmax,
    /// Key for 'net.inet.tcp.autotunereorder'
    NetInetTcpAutotunereorder,
    /// Key for 'net.inet.tcp.background_sockets'
    NetInetTcpBackground_sockets,
    /// Key for 'net.inet.tcp.backoff_maximum'
    NetInetTcpBackoff_maximum,
    /// Key for 'net.inet.tcp.bg_allowed_increase'
    NetInetTcpBg_allowed_increase,
    /// Key for 'net.inet.tcp.bg_ss_fltsz'
    NetInetTcpBg_ss_fltsz,
    /// Key for 'net.inet.tcp.bg_target_qdelay'
    NetInetTcpBg_target_qdelay,
    /// Key for 'net.inet.tcp.bg_tether_shift'
    NetInetTcpBg_tether_shift,
    /// Key for 'net.inet.tcp.blackhole'
    NetInetTcpBlackhole,
    /// Key for 'net.inet.tcp.broken_peer_syn_rexmit_thres'
    NetInetTcpBroken_peer_syn_rexmit_thres,
    /// Key for 'net.inet.tcp.cc_debug'
    NetInetTcpCc_debug,
    /// Key for 'net.inet.tcp.challengeack_limit'
    NetInetTcpChallengeack_limit,
    /// Key for 'net.inet.tcp.clear_tfocache'
    NetInetTcpClear_tfocache,
    /// Key for 'net.inet.tcp.cubic_fast_convergence'
    NetInetTcpCubic_fast_convergence,
    /// Key for 'net.inet.tcp.cubic_sockets'
    NetInetTcpCubic_sockets,
    /// Key for 'net.inet.tcp.cubic_tcp_friendliness'
    NetInetTcpCubic_tcp_friendliness,
    /// Key for 'net.inet.tcp.cubic_use_minrtt'
    NetInetTcpCubic_use_minrtt,
    /// Key for 'net.inet.tcp.delayed_ack'
    NetInetTcpDelayed_ack,
    /// Key for 'net.inet.tcp.disable_access_to_stats'
    NetInetTcpDisable_access_to_stats,
    /// Key for 'net.inet.tcp.disable_tcp_heuristics'
    NetInetTcpDisable_tcp_heuristics,
    /// Key for 'net.inet.tcp.do_rfc5961'
    NetInetTcpDo_rfc5961,
    /// Key for 'net.inet.tcp.do_tcpdrain'
    NetInetTcpDo_tcpdrain,
    /// Key for 'net.inet.tcp.doautorcvbuf'
    NetInetTcpDoautorcvbuf,
    /// Key for 'net.inet.tcp.doautosndbuf'
    NetInetTcpDoautosndbuf,
    /// Key for 'net.inet.tcp.drop_synfin'
    NetInetTcpDrop_synfin,
    /// Key for 'net.inet.tcp.ecn_initiate_out'
    NetInetTcpEcn_initiate_out,
    /// Key for 'net.inet.tcp.ecn_negotiate_in'
    NetInetTcpEcn_negotiate_in,
    /// Key for 'net.inet.tcp.ecn_setup_percentage'
    NetInetTcpEcn_setup_percentage,
    /// Key for 'net.inet.tcp.ecn_timeout'
    NetInetTcpEcn_timeout,
    /// Key for 'net.inet.tcp.enable_tlp'
    NetInetTcpEnable_tlp,
    /// Key for 'net.inet.tcp.fastopen'
    NetInetTcpFastopen,
    /// Key for 'net.inet.tcp.fastopen_backlog'
    NetInetTcpFastopen_backlog,
    /// Key for 'net.inet.tcp.icmp_may_rst'
    NetInetTcpIcmp_may_rst,
    /// Key for 'net.inet.tcp.init_rtt_from_cache'
    NetInetTcpInit_rtt_from_cache,
    /// Key for 'net.inet.tcp.keepcnt'
    NetInetTcpKeepcnt,
    /// Key for 'net.inet.tcp.keepidle'
    NetInetTcpKeepidle,
    /// Key for 'net.inet.tcp.keepinit'
    NetInetTcpKeepinit,
    /// Key for 'net.inet.tcp.keepintvl'
    NetInetTcpKeepintvl,
    /// Key for 'net.inet.tcp.local_slowstart_flightsize'
    NetInetTcpLocal_slowstart_flightsize,
    /// Key for 'net.inet.tcp.log.enable'
    NetInetTcpLogEnable,
    /// Key for 'net.inet.tcp.log.enable_usage'
    NetInetTcpLogEnable_usage,
    /// Key for 'net.inet.tcp.log.level_info'
    NetInetTcpLogLevel_info,
    /// Key for 'net.inet.tcp.log.privacy'
    NetInetTcpLogPrivacy,
    /// Key for 'net.inet.tcp.log.rate_current'
    NetInetTcpLogRate_current,
    /// Key for 'net.inet.tcp.log.rate_duration'
    NetInetTcpLogRate_duration,
    /// Key for 'net.inet.tcp.log.rate_exceeded_total'
    NetInetTcpLogRate_exceeded_total,
    /// Key for 'net.inet.tcp.log.rate_limit'
    NetInetTcpLogRate_limit,
    /// Key for 'net.inet.tcp.log.rate_max'
    NetInetTcpLogRate_max,
    /// Key for 'net.inet.tcp.log.rtt_port'
    NetInetTcpLogRtt_port,
    /// Key for 'net.inet.tcp.log.thflags_if_family'
    NetInetTcpLogThflags_if_family,
    /// Key for 'net.inet.tcp.log_in_vain'
    NetInetTcpLog_in_vain,
    /// Key for 'net.inet.tcp.lro'
    NetInetTcpLro,
    /// Key for 'net.inet.tcp.lro_startcnt'
    NetInetTcpLro_startcnt,
    /// Key for 'net.inet.tcp.lro_sz'
    NetInetTcpLro_sz,
    /// Key for 'net.inet.tcp.lro_time'
    NetInetTcpLro_time,
    /// Key for 'net.inet.tcp.lrodbg'
    NetInetTcpLrodbg,
    /// Key for 'net.inet.tcp.max_persist_timeout'
    NetInetTcpMax_persist_timeout,
    /// Key for 'net.inet.tcp.maxseg_unacked'
    NetInetTcpMaxseg_unacked,
    /// Key for 'net.inet.tcp.microuptime_init'
    NetInetTcpMicrouptime_init,
    /// Key for 'net.inet.tcp.min_iaj_win'
    NetInetTcpMin_iaj_win,
    /// Key for 'net.inet.tcp.minmss'
    NetInetTcpMinmss,
    /// Key for 'net.inet.tcp.msl'
    NetInetTcpMsl,
    /// Key for 'net.inet.tcp.mssdflt'
    NetInetTcpMssdflt,
    /// Key for 'net.inet.tcp.newreno_sockets'
    NetInetTcpNewreno_sockets,
    /// Key for 'net.inet.tcp.now_init'
    NetInetTcpNow_init,
    /// Key for 'net.inet.tcp.packetchain'
    NetInetTcpPacketchain,
    /// Key for 'net.inet.tcp.path_mtu_discovery'
    NetInetTcpPath_mtu_discovery,
    /// Key for 'net.inet.tcp.pcbcount'
    NetInetTcpPcbcount,
    /// Key for 'net.inet.tcp.pmtud_blackhole_detection'
    NetInetTcpPmtud_blackhole_detection,
    /// Key for 'net.inet.tcp.pmtud_blackhole_mss'
    NetInetTcpPmtud_blackhole_mss,
    /// Key for 'net.inet.tcp.randomize_ports'
    NetInetTcpRandomize_ports,
    /// Key for 'net.inet.tcp.rcvsspktcnt'
    NetInetTcpRcvsspktcnt,
    /// Key for 'net.inet.tcp.reass.overflows'
    NetInetTcpReassOverflows,
    /// Key for 'net.inet.tcp.recv_allowed_iaj'
    NetInetTcpRecv_allowed_iaj,
    /// Key for 'net.inet.tcp.recv_throttle_minwin'
    NetInetTcpRecv_throttle_minwin,
    /// Key for 'net.inet.tcp.recvbg'
    NetInetTcpRecvbg,
    /// Key for 'net.inet.tcp.recvspace'
    NetInetTcpRecvspace,
    /// Key for 'net.inet.tcp.rexmt_slop'
    NetInetTcpRexmt_slop,
    /// Key for 'net.inet.tcp.rexmt_thresh'
    NetInetTcpRexmt_thresh,
    /// Key for 'net.inet.tcp.rfc1644'
    NetInetTcpRfc1644,
    /// Key for 'net.inet.tcp.rfc3390'
    NetInetTcpRfc3390,
    /// Key for 'net.inet.tcp.rfc3465'
    NetInetTcpRfc3465,
    /// Key for 'net.inet.tcp.rfc3465_lim2'
    NetInetTcpRfc3465_lim2,
    /// Key for 'net.inet.tcp.rtt_min'
    NetInetTcpRtt_min,
    /// Key for 'net.inet.tcp.rtt_recvbg'
    NetInetTcpRtt_recvbg,
    /// Key for 'net.inet.tcp.sack'
    NetInetTcpSack,
    /// Key for 'net.inet.tcp.sack_globalholes'
    NetInetTcpSack_globalholes,
    /// Key for 'net.inet.tcp.sack_globalmaxholes'
    NetInetTcpSack_globalmaxholes,
    /// Key for 'net.inet.tcp.sack_maxholes'
    NetInetTcpSack_maxholes,
    /// Key for 'net.inet.tcp.sendspace'
    NetInetTcpSendspace,
    /// Key for 'net.inet.tcp.slowlink_wsize'
    NetInetTcpSlowlink_wsize,
    /// Key for 'net.inet.tcp.slowstart_flightsize'
    NetInetTcpSlowstart_flightsize,
    /// Key for 'net.inet.tcp.socket_unlocked_on_output'
    NetInetTcpSocket_unlocked_on_output,
    /// Key for 'net.inet.tcp.tcbhashsize'
    NetInetTcpTcbhashsize,
    /// Key for 'net.inet.tcp.tcp_lq_overflow'
    NetInetTcpTcp_lq_overflow,
    /// Key for 'net.inet.tcp.tcp_resched_timerlist'
    NetInetTcpTcp_resched_timerlist,
    /// Key for 'net.inet.tcp.tcp_timer_advanced'
    NetInetTcpTcp_timer_advanced,
    /// Key for 'net.inet.tcp.timer_fastmode_idlemax'
    NetInetTcpTimer_fastmode_idlemax,
    /// Key for 'net.inet.tcp.tso'
    NetInetTcpTso,
    /// Key for 'net.inet.tcp.tw_pcbcount'
    NetInetTcpTw_pcbcount,
    /// Key for 'net.inet.tcp.use_newreno'
    NetInetTcpUse_newreno,
    /// Key for 'net.inet.tcp.v6mssdflt'
    NetInetTcpV6mssdflt,
    /// Key for 'net.inet.tcp.win_scale_factor'
    NetInetTcpWin_scale_factor,
    /// Key for 'net.inet.udp.blackhole'
    NetInetUdpBlackhole,
    /// Key for 'net.inet.udp.checksum'
    NetInetUdpChecksum,
    /// Key for 'net.inet.udp.log_in_vain'
    NetInetUdpLog_in_vain,
    /// Key for 'net.inet.udp.maxdgram'
    NetInetUdpMaxdgram,
    /// Key for 'net.inet.udp.pcbcount'
    NetInetUdpPcbcount,
    /// Key for 'net.inet.udp.randomize_ports'
    NetInetUdpRandomize_ports,
    /// Key for 'net.inet.udp.recvspace'
    NetInetUdpRecvspace,
    /// Key for 'net.ipsec.debug'
    NetIpsecDebug,
    /// Key for 'net.ipsec.max_pending_input'
    NetIpsecMax_pending_input,
    /// Key for 'net.ipsec.ring_size'
    NetIpsecRing_size,
    /// Key for 'net.ipsec.rx_fsw_ring_size'
    NetIpsecRx_fsw_ring_size,
    /// Key for 'net.ipsec.tx_fsw_ring_size'
    NetIpsecTx_fsw_ring_size,
    /// Key for 'net.ipsec.verify_interface_creation'
    NetIpsecVerify_interface_creation,
    /// Key for 'net.key.ah_keymin'
    NetKeyAh_keymin,
    /// Key for 'net.key.blockacq_count'
    NetKeyBlockacq_count,
    /// Key for 'net.key.blockacq_lifetime'
    NetKeyBlockacq_lifetime,
    /// Key for 'net.key.debug'
    NetKeyDebug,
    /// Key for 'net.key.esp_auth'
    NetKeyEsp_auth,
    /// Key for 'net.key.esp_keymin'
    NetKeyEsp_keymin,
    /// Key for 'net.key.int_random'
    NetKeyInt_random,
    /// Key for 'net.key.larval_lifetime'
    NetKeyLarval_lifetime,
    /// Key for 'net.key.natt_keepalive_interval'
    NetKeyNatt_keepalive_interval,
    /// Key for 'net.key.prefered_oldsa'
    NetKeyPrefered_oldsa,
    /// Key for 'net.key.spi_maxval'
    NetKeySpi_maxval,
    /// Key for 'net.key.spi_minval'
    NetKeySpi_minval,
    /// Key for 'net.key.spi_trycnt'
    NetKeySpi_trycnt,
    /// Key for 'net.link.bond.debug'
    NetLinkBondDebug,
    /// Key for 'net.link.bridge.debug'
    NetLinkBridgeDebug,
    /// Key for 'net.link.bridge.inherit_mac'
    NetLinkBridgeInherit_mac,
    /// Key for 'net.link.bridge.rtable_hash_size_max'
    NetLinkBridgeRtable_hash_size_max,
    /// Key for 'net.link.bridge.rtable_prune_period'
    NetLinkBridgeRtable_prune_period,
    /// Key for 'net.link.bridge.txstart'
    NetLinkBridgeTxstart,
    /// Key for 'net.link.ether.inet.arp_llreach_base'
    NetLinkEtherInetArp_llreach_base,
    /// Key for 'net.link.ether.inet.arp_unicast_lim'
    NetLinkEtherInetArp_unicast_lim,
    /// Key for 'net.link.ether.inet.host_down_time'
    NetLinkEtherInetHost_down_time,
    /// Key for 'net.link.ether.inet.keep_announcements'
    NetLinkEtherInetKeep_announcements,
    /// Key for 'net.link.ether.inet.log_arp_warnings'
    NetLinkEtherInetLog_arp_warnings,
    /// Key for 'net.link.ether.inet.max_age'
    NetLinkEtherInetMax_age,
    /// Key for 'net.link.ether.inet.maxhold'
    NetLinkEtherInetMaxhold,
    /// Key for 'net.link.ether.inet.maxhold_total'
    NetLinkEtherInetMaxhold_total,
    /// Key for 'net.link.ether.inet.maxtries'
    NetLinkEtherInetMaxtries,
    /// Key for 'net.link.ether.inet.probe_intvl'
    NetLinkEtherInetProbe_intvl,
    /// Key for 'net.link.ether.inet.proxyall'
    NetLinkEtherInetProxyall,
    /// Key for 'net.link.ether.inet.prune_intvl'
    NetLinkEtherInetPrune_intvl,
    /// Key for 'net.link.ether.inet.send_conflicting_probes'
    NetLinkEtherInetSend_conflicting_probes,
    /// Key for 'net.link.ether.inet.sendllconflict'
    NetLinkEtherInetSendllconflict,
    /// Key for 'net.link.ether.inet.useloopback'
    NetLinkEtherInetUseloopback,
    /// Key for 'net.link.ether.inet.verbose'
    NetLinkEtherInetVerbose,
    /// Key for 'net.link.fake.bsd_mode'
    NetLinkFakeBsd_mode,
    /// Key for 'net.link.fake.buflet_size'
    NetLinkFakeBuflet_size,
    /// Key for 'net.link.fake.copypkt_mode'
    NetLinkFakeCopypkt_mode,
    /// Key for 'net.link.fake.debug'
    NetLinkFakeDebug,
    /// Key for 'net.link.fake.hwcsum'
    NetLinkFakeHwcsum,
    /// Key for 'net.link.fake.if_adv_intvl'
    NetLinkFakeIf_adv_intvl,
    /// Key for 'net.link.fake.max_mtu'
    NetLinkFakeMax_mtu,
    /// Key for 'net.link.fake.multibuflet'
    NetLinkFakeMultibuflet,
    /// Key for 'net.link.fake.nxattach'
    NetLinkFakeNxattach,
    /// Key for 'net.link.fake.tx_drops'
    NetLinkFakeTx_drops,
    /// Key for 'net.link.fake.tx_headroom'
    NetLinkFakeTx_headroom,
    /// Key for 'net.link.fake.txstart'
    NetLinkFakeTxstart,
    /// Key for 'net.link.fake.user_access'
    NetLinkFakeUser_access,
    /// Key for 'net.link.fake.wmm_mode'
    NetLinkFakeWmm_mode,
    /// Key for 'net.link.generic.system.delaybased_queue'
    NetLinkGenericSystemDelaybased_queue,
    /// Key for 'net.link.generic.system.dlil_input_sanity_check'
    NetLinkGenericSystemDlil_input_sanity_check,
    /// Key for 'net.link.generic.system.dlil_input_threads'
    NetLinkGenericSystemDlil_input_threads,
    /// Key for 'net.link.generic.system.dlil_verbose'
    NetLinkGenericSystemDlil_verbose,
    /// Key for 'net.link.generic.system.enable_netagent'
    NetLinkGenericSystemEnable_netagent,
    /// Key for 'net.link.generic.system.flow_advisory'
    NetLinkGenericSystemFlow_advisory,
    /// Key for 'net.link.generic.system.hwcksum_dbg'
    NetLinkGenericSystemHwcksum_dbg,
    /// Key for 'net.link.generic.system.hwcksum_dbg_adjusted'
    NetLinkGenericSystemHwcksum_dbg_adjusted,
    /// Key for 'net.link.generic.system.hwcksum_dbg_bad_cksum'
    NetLinkGenericSystemHwcksum_dbg_bad_cksum,
    /// Key for 'net.link.generic.system.hwcksum_dbg_bad_rxoff'
    NetLinkGenericSystemHwcksum_dbg_bad_rxoff,
    /// Key for 'net.link.generic.system.hwcksum_dbg_finalized_data'
    NetLinkGenericSystemHwcksum_dbg_finalized_data,
    /// Key for 'net.link.generic.system.hwcksum_dbg_finalized_hdr'
    NetLinkGenericSystemHwcksum_dbg_finalized_hdr,
    /// Key for 'net.link.generic.system.hwcksum_dbg_mode'
    NetLinkGenericSystemHwcksum_dbg_mode,
    /// Key for 'net.link.generic.system.hwcksum_dbg_partial_forced'
    NetLinkGenericSystemHwcksum_dbg_partial_forced,
    /// Key for 'net.link.generic.system.hwcksum_dbg_partial_forced_bytes'
    NetLinkGenericSystemHwcksum_dbg_partial_forced_bytes,
    /// Key for 'net.link.generic.system.hwcksum_dbg_partial_rxoff_adj'
    NetLinkGenericSystemHwcksum_dbg_partial_rxoff_adj,
    /// Key for 'net.link.generic.system.hwcksum_dbg_partial_rxoff_forced'
    NetLinkGenericSystemHwcksum_dbg_partial_rxoff_forced,
    /// Key for 'net.link.generic.system.hwcksum_dbg_verified'
    NetLinkGenericSystemHwcksum_dbg_verified,
    /// Key for 'net.link.generic.system.hwcksum_in_invalidated'
    NetLinkGenericSystemHwcksum_in_invalidated,
    /// Key for 'net.link.generic.system.hwcksum_rx'
    NetLinkGenericSystemHwcksum_rx,
    /// Key for 'net.link.generic.system.hwcksum_tx'
    NetLinkGenericSystemHwcksum_tx,
    /// Key for 'net.link.generic.system.if_verbose'
    NetLinkGenericSystemIf_verbose,
    /// Key for 'net.link.generic.system.ifcount'
    NetLinkGenericSystemIfcount,
    /// Key for 'net.link.generic.system.port_used.entry_count'
    NetLinkGenericSystemPort_usedEntry_count,
    /// Key for 'net.link.generic.system.port_used.entry_gen'
    NetLinkGenericSystemPort_usedEntry_gen,
    /// Key for 'net.link.generic.system.port_used.verbose'
    NetLinkGenericSystemPort_usedVerbose,
    /// Key for 'net.link.generic.system.port_used.wakeuuid_not_set_count'
    NetLinkGenericSystemPort_usedWakeuuid_not_set_count,
    /// Key for 'net.link.generic.system.port_used.wakeuuid_not_set_last_if'
    NetLinkGenericSystemPort_usedWakeuuid_not_set_last_if,
    /// Key for 'net.link.generic.system.port_used.wakeuuid_not_set_last_time'
    NetLinkGenericSystemPort_usedWakeuuid_not_set_last_time,
    /// Key for 'net.link.generic.system.rcvq_maxlen'
    NetLinkGenericSystemRcvq_maxlen,
    /// Key for 'net.link.generic.system.rxpoll'
    NetLinkGenericSystemRxpoll,
    /// Key for 'net.link.generic.system.rxpoll_decay'
    NetLinkGenericSystemRxpoll_decay,
    /// Key for 'net.link.generic.system.rxpoll_freeze_time'
    NetLinkGenericSystemRxpoll_freeze_time,
    /// Key for 'net.link.generic.system.rxpoll_interval_pkts'
    NetLinkGenericSystemRxpoll_interval_pkts,
    /// Key for 'net.link.generic.system.rxpoll_interval_time'
    NetLinkGenericSystemRxpoll_interval_time,
    /// Key for 'net.link.generic.system.rxpoll_max'
    NetLinkGenericSystemRxpoll_max,
    /// Key for 'net.link.generic.system.rxpoll_sample_time'
    NetLinkGenericSystemRxpoll_sample_time,
    /// Key for 'net.link.generic.system.rxpoll_wakeups_hiwat'
    NetLinkGenericSystemRxpoll_wakeups_hiwat,
    /// Key for 'net.link.generic.system.rxpoll_wakeups_lowat'
    NetLinkGenericSystemRxpoll_wakeups_lowat,
    /// Key for 'net.link.generic.system.sndq_maxlen'
    NetLinkGenericSystemSndq_maxlen,
    /// Key for 'net.link.generic.system.start_delay_disabled'
    NetLinkGenericSystemStart_delay_disabled,
    /// Key for 'net.link.generic.system.start_delayed'
    NetLinkGenericSystemStart_delayed,
    /// Key for 'net.link.generic.system.threshold_interval'
    NetLinkGenericSystemThreshold_interval,
    /// Key for 'net.link.generic.system.threshold_notify'
    NetLinkGenericSystemThreshold_notify,
    /// Key for 'net.link.generic.system.tx_chain_len_count'
    NetLinkGenericSystemTx_chain_len_count,
    /// Key for 'net.link.iptap.log'
    NetLinkIptapLog,
    /// Key for 'net.link.iptap.total_tap_count'
    NetLinkIptapTotal_tap_count,
    /// Key for 'net.link.loopback.dequeue_sc'
    NetLinkLoopbackDequeue_sc,
    /// Key for 'net.link.loopback.max_dequeue'
    NetLinkLoopbackMax_dequeue,
    /// Key for 'net.link.loopback.sched_model'
    NetLinkLoopbackSched_model,
    /// Key for 'net.link.pktap.count_unknown_if_type'
    NetLinkPktapCount_unknown_if_type,
    /// Key for 'net.link.pktap.log'
    NetLinkPktapLog,
    /// Key for 'net.link.pktap.total_tap_count'
    NetLinkPktapTotal_tap_count,
    /// Key for 'net.local.dgram.maxdgram'
    NetLocalDgramMaxdgram,
    /// Key for 'net.local.dgram.recvspace'
    NetLocalDgramRecvspace,
    /// Key for 'net.local.inflight'
    NetLocalInflight,
    /// Key for 'net.local.stream.recvspace'
    NetLocalStreamRecvspace,
    /// Key for 'net.local.stream.sendspace'
    NetLocalStreamSendspace,
    /// Key for 'net.local.stream.tracemdns'
    NetLocalStreamTracemdns,
    /// Key for 'net.mpklog.enabled'
    NetMpklogEnabled,
    /// Key for 'net.mpklog.type'
    NetMpklogType,
    /// Key for 'net.mpklog.version'
    NetMpklogVersion,
    /// Key for 'net.ndrv_multi_max_count'
    NetNdrv_multi_max_count,
    /// Key for 'net.necp.arena_count'
    NetNecpArena_count,
    /// Key for 'net.necp.client_count'
    NetNecpClient_count,
    /// Key for 'net.necp.client_fd_count'
    NetNecpClient_fd_count,
    /// Key for 'net.necp.debug'
    NetNecpDebug,
    /// Key for 'net.necp.drop_all_level'
    NetNecpDrop_all_level,
    /// Key for 'net.necp.drop_dest_debug'
    NetNecpDrop_dest_debug,
    /// Key for 'net.necp.drop_unentitled_level'
    NetNecpDrop_unentitled_level,
    /// Key for 'net.necp.if_flow_count'
    NetNecpIf_flow_count,
    /// Key for 'net.necp.ip_policy_count'
    NetNecpIp_policy_count,
    /// Key for 'net.necp.nexus_flow_count'
    NetNecpNexus_flow_count,
    /// Key for 'net.necp.observer_fd_count'
    NetNecpObserver_fd_count,
    /// Key for 'net.necp.observer_message_limit'
    NetNecpObserver_message_limit,
    /// Key for 'net.necp.pass_interpose'
    NetNecpPass_interpose,
    /// Key for 'net.necp.pass_keepalives'
    NetNecpPass_keepalives,
    /// Key for 'net.necp.pass_loopback'
    NetNecpPass_loopback,
    /// Key for 'net.necp.session_count'
    NetNecpSession_count,
    /// Key for 'net.necp.socket_flow_count'
    NetNecpSocket_flow_count,
    /// Key for 'net.necp.socket_non_app_policy_count'
    NetNecpSocket_non_app_policy_count,
    /// Key for 'net.necp.socket_policy_count'
    NetNecpSocket_policy_count,
    /// Key for 'net.necp.sysctl_arena_count'
    NetNecpSysctl_arena_count,
    /// Key for 'net.netagent.active_count'
    NetNetagentActive_count,
    /// Key for 'net.netagent.debug'
    NetNetagentDebug,
    /// Key for 'net.netagent.registered_count'
    NetNetagentRegistered_count,
    /// Key for 'net.pktsched.verbose'
    NetPktschedVerbose,
    /// Key for 'net.qos.policy.capable_enabled'
    NetQosPolicyCapable_enabled,
    /// Key for 'net.qos.policy.restrict_avapps'
    NetQosPolicyRestrict_avapps,
    /// Key for 'net.qos.policy.restricted'
    NetQosPolicyRestricted,
    /// Key for 'net.qos.policy.wifi_enabled'
    NetQosPolicyWifi_enabled,
    /// Key for 'net.qos.reset_dscp_to_wifi_ac_map'
    NetQosReset_dscp_to_wifi_ac_map,
    /// Key for 'net.qos.verbose'
    NetQosVerbose,
    /// Key for 'net.restricted_port.enforced'
    NetRestricted_portEnforced,
    /// Key for 'net.restricted_port.verbose'
    NetRestricted_portVerbose,
    /// Key for 'net.smb.fs.kern_deadtimer'
    NetSmbFsKern_deadtimer,
    /// Key for 'net.smb.fs.kern_hard_deadtimer'
    NetSmbFsKern_hard_deadtimer,
    /// Key for 'net.smb.fs.kern_soft_deadtimer'
    NetSmbFsKern_soft_deadtimer,
    /// Key for 'net.smb.fs.loglevel'
    NetSmbFsLoglevel,
    /// Key for 'net.smb.fs.maxread'
    NetSmbFsMaxread,
    /// Key for 'net.smb.fs.maxsegreadsize'
    NetSmbFsMaxsegreadsize,
    /// Key for 'net.smb.fs.maxsegwritesize'
    NetSmbFsMaxsegwritesize,
    /// Key for 'net.smb.fs.maxwrite'
    NetSmbFsMaxwrite,
    /// Key for 'net.smb.fs.tcprcvbuf'
    NetSmbFsTcprcvbuf,
    /// Key for 'net.smb.fs.tcpsndbuf'
    NetSmbFsTcpsndbuf,
    /// Key for 'net.smb.fs.version'
    NetSmbFsVersion,
    /// Key for 'net.statistics_privcheck'
    NetStatistics_privcheck,
    /// Key for 'net.stats.debug'
    NetStatsDebug,
    /// Key for 'net.stats.recvspace'
    NetStatsRecvspace,
    /// Key for 'net.stats.sendspace'
    NetStatsSendspace,
    /// Key for 'net.systm.kctl.autorcvbufhigh'
    NetSystmKctlAutorcvbufhigh,
    /// Key for 'net.systm.kctl.autorcvbufmax'
    NetSystmKctlAutorcvbufmax,
    /// Key for 'net.systm.kctl.debug'
    NetSystmKctlDebug,
    /// Key for 'net.utun.max_pending_input'
    NetUtunMax_pending_input,
    /// Key for 'net.utun.ring_size'
    NetUtunRing_size,
    /// Key for 'net.utun.rx_fsw_ring_size'
    NetUtunRx_fsw_ring_size,
    /// Key for 'net.utun.tx_fsw_ring_size'
    NetUtunTx_fsw_ring_size,
    /// Key for 'security.mac.amfi.force_policy'
    SecurityMacAmfiForce_policy,
    /// Key for 'security.mac.amfi.hsp_enable'
    SecurityMacAmfiHsp_enable,
    /// Key for 'security.mac.amfi.verbose_logging'
    SecurityMacAmfiVerbose_logging,
    /// Key for 'security.mac.asp.active_rule_version'
    SecurityMacAspActive_rule_version,
    /// Key for 'security.mac.asp.cache_allocation_count'
    SecurityMacAspCache_allocation_count,
    /// Key for 'security.mac.asp.cache_entry_count'
    SecurityMacAspCache_entry_count,
    /// Key for 'security.mac.asp.cache_release_count'
    SecurityMacAspCache_release_count,
    /// Key for 'security.mac.asp.exec_hook_count'
    SecurityMacAspExec_hook_count,
    /// Key for 'security.mac.asp.exec_hook_sleep_time'
    SecurityMacAspExec_hook_sleep_time,
    /// Key for 'security.mac.asp.exec_hook_work_time'
    SecurityMacAspExec_hook_work_time,
    /// Key for 'security.mac.asp.library_hook_count'
    SecurityMacAspLibrary_hook_count,
    /// Key for 'security.mac.asp.library_hook_time'
    SecurityMacAspLibrary_hook_time,
    /// Key for 'security.mac.asp.library_sleep_time'
    SecurityMacAspLibrary_sleep_time,
    /// Key for 'security.mac.asp.pending_evaluation_count'
    SecurityMacAspPending_evaluation_count,
    /// Key for 'security.mac.device_enforce'
    SecurityMacDevice_enforce,
    /// Key for 'security.mac.endpointsecurity.log_level'
    SecurityMacEndpointsecurityLog_level,
    /// Key for 'security.mac.labelvnodes'
    SecurityMacLabelvnodes,
    /// Key for 'security.mac.max_slots'
    SecurityMacMax_slots,
    /// Key for 'security.mac.pipe_enforce'
    SecurityMacPipe_enforce,
    /// Key for 'security.mac.platform_exec_logging'
    SecurityMacPlatform_exec_logging,
    /// Key for 'security.mac.posixsem_enforce'
    SecurityMacPosixsem_enforce,
    /// Key for 'security.mac.posixshm_enforce'
    SecurityMacPosixshm_enforce,
    /// Key for 'security.mac.proc_enforce'
    SecurityMacProc_enforce,
    /// Key for 'security.mac.qtn.sandbox_enforce'
    SecurityMacQtnSandbox_enforce,
    /// Key for 'security.mac.qtn.user_approved_exec'
    SecurityMacQtnUser_approved_exec,
    /// Key for 'security.mac.sandbox.audio_active'
    SecurityMacSandboxAudio_active,
    /// Key for 'security.mac.sandbox.sentinel'
    SecurityMacSandboxSentinel,
    /// Key for 'security.mac.socket_enforce'
    SecurityMacSocket_enforce,
    /// Key for 'security.mac.system_enforce'
    SecurityMacSystem_enforce,
    /// Key for 'security.mac.sysvmsg_enforce'
    SecurityMacSysvmsg_enforce,
    /// Key for 'security.mac.sysvsem_enforce'
    SecurityMacSysvsem_enforce,
    /// Key for 'security.mac.sysvshm_enforce'
    SecurityMacSysvshm_enforce,
    /// Key for 'security.mac.vm_enforce'
    SecurityMacVm_enforce,
    /// Key for 'security.mac.vnode_enforce'
    SecurityMacVnode_enforce,
    /// Key for 'security.mac.vnode_label_count'
    SecurityMacVnode_label_count,
    /// Key for 'user.bc_base_max'
    UserBc_base_max,
    /// Key for 'user.bc_dim_max'
    UserBc_dim_max,
    /// Key for 'user.bc_scale_max'
    UserBc_scale_max,
    /// Key for 'user.bc_string_max'
    UserBc_string_max,
    /// Key for 'user.coll_weights_max'
    UserColl_weights_max,
    /// Key for 'user.cs_path'
    UserCs_path,
    /// Key for 'user.expr_nest_max'
    UserExpr_nest_max,
    /// Key for 'user.line_max'
    UserLine_max,
    /// Key for 'user.posix2_c_bind'
    UserPosix2_c_bind,
    /// Key for 'user.posix2_c_dev'
    UserPosix2_c_dev,
    /// Key for 'user.posix2_char_term'
    UserPosix2_char_term,
    /// Key for 'user.posix2_fort_dev'
    UserPosix2_fort_dev,
    /// Key for 'user.posix2_fort_run'
    UserPosix2_fort_run,
    /// Key for 'user.posix2_localedef'
    UserPosix2_localedef,
    /// Key for 'user.posix2_sw_dev'
    UserPosix2_sw_dev,
    /// Key for 'user.posix2_upe'
    UserPosix2_upe,
    /// Key for 'user.posix2_version'
    UserPosix2_version,
    /// Key for 'user.re_dup_max'
    UserRe_dup_max,
    /// Key for 'user.stream_max'
    UserStream_max,
    /// Key for 'user.tzname_max'
    UserTzname_max,
    /// Key for 'vfs.generic.always_do_fullfsync'
    VfsGenericAlways_do_fullfsync,
    /// Key for 'vfs.generic.apfs.allocated'
    VfsGenericApfsAllocated,
    /// Key for 'vfs.generic.apfs.fusion_elevator_throttle'
    VfsGenericApfsFusion_elevator_throttle,
    /// Key for 'vfs.generic.apfs.fusion_lc_rc_promotion_threshold_mult'
    VfsGenericApfsFusion_lc_rc_promotion_threshold_mult,
    /// Key for 'vfs.generic.apfs.fusion_paranoia_level'
    VfsGenericApfsFusion_paranoia_level,
    /// Key for 'vfs.generic.apfs.fusion_promoter_queue_limit'
    VfsGenericApfsFusion_promoter_queue_limit,
    /// Key for 'vfs.generic.apfs.fusion_promoter_throttle'
    VfsGenericApfsFusion_promoter_throttle,
    /// Key for 'vfs.generic.apfs.fusion_rc_flags'
    VfsGenericApfsFusion_rc_flags,
    /// Key for 'vfs.generic.apfs.fusion_rc_promotion_size_limit_mb'
    VfsGenericApfsFusion_rc_promotion_size_limit_mb,
    /// Key for 'vfs.generic.apfs.fusion_rc_promotion_threshold_mult'
    VfsGenericApfsFusion_rc_promotion_threshold_mult,
    /// Key for 'vfs.generic.apfs.fusion_slow_io_threshold'
    VfsGenericApfsFusion_slow_io_threshold,
    /// Key for 'vfs.generic.apfs.fusion_swapfile_backoff'
    VfsGenericApfsFusion_swapfile_backoff,
    /// Key for 'vfs.generic.apfs.fusion_verbosity_flags'
    VfsGenericApfsFusion_verbosity_flags,
    /// Key for 'vfs.generic.apfs.fusion_w2rc_filled_ratio_threshold'
    VfsGenericApfsFusion_w2rc_filled_ratio_threshold,
    /// Key for 'vfs.generic.apfs.fusion_wbc_backoff_wmk_low'
    VfsGenericApfsFusion_wbc_backoff_wmk_low,
    /// Key for 'vfs.generic.apfs.fusion_wbc_backoff_wmk_med'
    VfsGenericApfsFusion_wbc_backoff_wmk_med,
    /// Key for 'vfs.generic.apfs.fusion_wbc_backoff_wmk_reenable'
    VfsGenericApfsFusion_wbc_backoff_wmk_reenable,
    /// Key for 'vfs.generic.apfs.fusion_wbc_buffersize'
    VfsGenericApfsFusion_wbc_buffersize,
    /// Key for 'vfs.generic.apfs.fusion_wbc_elevator_wmk'
    VfsGenericApfsFusion_wbc_elevator_wmk,
    /// Key for 'vfs.generic.autofs.vnode_recycle_on_inactive'
    VfsGenericAutofsVnode_recycle_on_inactive,
    /// Key for 'vfs.generic.hfs.allocated'
    VfsGenericHfsAllocated,
    /// Key for 'vfs.generic.hfs.jnl.kdebug.trim'
    VfsGenericHfsJnlKdebugTrim,
    /// Key for 'vfs.generic.hfs.jnl.trim_flush'
    VfsGenericHfsJnlTrim_flush,
    /// Key for 'vfs.generic.hfs.kdebug.allocation'
    VfsGenericHfsKdebugAllocation,
    /// Key for 'vfs.generic.maxtypenum'
    VfsGenericMaxtypenum,
    /// Key for 'vfs.generic.nfs.client.access_cache_timeout'
    VfsGenericNfsClientAccess_cache_timeout,
    /// Key for 'vfs.generic.nfs.client.access_delete'
    VfsGenericNfsClientAccess_delete,
    /// Key for 'vfs.generic.nfs.client.access_dotzfs'
    VfsGenericNfsClientAccess_dotzfs,
    /// Key for 'vfs.generic.nfs.client.access_for_getattr'
    VfsGenericNfsClientAccess_for_getattr,
    /// Key for 'vfs.generic.nfs.client.allow_async'
    VfsGenericNfsClientAllow_async,
    /// Key for 'vfs.generic.nfs.client.callback_port'
    VfsGenericNfsClientCallback_port,
    /// Key for 'vfs.generic.nfs.client.debug_ctl'
    VfsGenericNfsClientDebug_ctl,
    /// Key for 'vfs.generic.nfs.client.default_nfs4domain'
    VfsGenericNfsClientDefault_nfs4domain,
    /// Key for 'vfs.generic.nfs.client.idmap_ctrl'
    VfsGenericNfsClientIdmap_ctrl,
    /// Key for 'vfs.generic.nfs.client.initialdowndelay'
    VfsGenericNfsClientInitialdowndelay,
    /// Key for 'vfs.generic.nfs.client.iosize'
    VfsGenericNfsClientIosize,
    /// Key for 'vfs.generic.nfs.client.is_mobile'
    VfsGenericNfsClientIs_mobile,
    /// Key for 'vfs.generic.nfs.client.lockd_mounts'
    VfsGenericNfsClientLockd_mounts,
    /// Key for 'vfs.generic.nfs.client.max_async_writes'
    VfsGenericNfsClientMax_async_writes,
    /// Key for 'vfs.generic.nfs.client.nextdowndelay'
    VfsGenericNfsClientNextdowndelay,
    /// Key for 'vfs.generic.nfs.client.nfsiod_thread_count'
    VfsGenericNfsClientNfsiod_thread_count,
    /// Key for 'vfs.generic.nfs.client.nfsiod_thread_max'
    VfsGenericNfsClientNfsiod_thread_max,
    /// Key for 'vfs.generic.nfs.client.readlink_nocache'
    VfsGenericNfsClientReadlink_nocache,
    /// Key for 'vfs.generic.nfs.client.root_steals_gss_context'
    VfsGenericNfsClientRoot_steals_gss_context,
    /// Key for 'vfs.generic.nfs.client.squishy_flags'
    VfsGenericNfsClientSquishy_flags,
    /// Key for 'vfs.generic.nfs.client.statfs_rate_limit'
    VfsGenericNfsClientStatfs_rate_limit,
    /// Key for 'vfs.generic.nfs.server.async'
    VfsGenericNfsServerAsync,
    /// Key for 'vfs.generic.nfs.server.export_hash_size'
    VfsGenericNfsServerExport_hash_size,
    /// Key for 'vfs.generic.nfs.server.fsevents'
    VfsGenericNfsServerFsevents,
    /// Key for 'vfs.generic.nfs.server.gss_context_ttl'
    VfsGenericNfsServerGss_context_ttl,
    /// Key for 'vfs.generic.nfs.server.nfsd_sock_idle_timeout'
    VfsGenericNfsServerNfsd_sock_idle_timeout,
    /// Key for 'vfs.generic.nfs.server.nfsd_tcp_connections'
    VfsGenericNfsServerNfsd_tcp_connections,
    /// Key for 'vfs.generic.nfs.server.nfsd_thread_count'
    VfsGenericNfsServerNfsd_thread_count,
    /// Key for 'vfs.generic.nfs.server.nfsd_thread_max'
    VfsGenericNfsServerNfsd_thread_max,
    /// Key for 'vfs.generic.nfs.server.reqcache_size'
    VfsGenericNfsServerReqcache_size,
    /// Key for 'vfs.generic.nfs.server.request_queue_length'
    VfsGenericNfsServerRequest_queue_length,
    /// Key for 'vfs.generic.nfs.server.require_resv_port'
    VfsGenericNfsServerRequire_resv_port,
    /// Key for 'vfs.generic.nfs.server.upcall_queue_count'
    VfsGenericNfsServerUpcall_queue_count,
    /// Key for 'vfs.generic.nfs.server.upcall_queue_limit'
    VfsGenericNfsServerUpcall_queue_limit,
    /// Key for 'vfs.generic.nfs.server.upcall_queue_max_seen'
    VfsGenericNfsServerUpcall_queue_max_seen,
    /// Key for 'vfs.generic.nfs.server.use_upcall_svc'
    VfsGenericNfsServerUse_upcall_svc,
    /// Key for 'vfs.generic.nfs.server.user_stats'
    VfsGenericNfsServerUser_stats,
    /// Key for 'vfs.generic.nfs.server.wg_delay'
    VfsGenericNfsServerWg_delay,
    /// Key for 'vfs.generic.nfs.server.wg_delay_v3'
    VfsGenericNfsServerWg_delay_v3,
    /// Key for 'vfs.generic.root_unmounted_cleanly'
    VfsGenericRoot_unmounted_cleanly,
    /// Key for 'vfs.generic.sync_timeout'
    VfsGenericSync_timeout,
    /// Key for 'vfs.nspace.prevent_materialization'
    VfsNspacePrevent_materialization,
    /// Key for 'vfs.nspace.resolver'
    VfsNspaceResolver,
    /// Key for 'vfs.nspace.thread_prevent_materialization'
    VfsNspaceThread_prevent_materialization,
    /// Key for 'vfs.nummntops'
    VfsNummntops,
    /// Key for 'vm.all_reusable_calls'
    VmAll_reusable_calls,
    /// Key for 'vm.all_reuse_calls'
    VmAll_reuse_calls,
    /// Key for 'vm.can_reuse_failure'
    VmCan_reuse_failure,
    /// Key for 'vm.can_reuse_success'
    VmCan_reuse_success,
    /// Key for 'vm.compressor_available'
    VmCompressor_available,
    /// Key for 'vm.compressor_bytes_used'
    VmCompressor_bytes_used,
    /// Key for 'vm.compressor_compressed_bytes'
    VmCompressor_compressed_bytes,
    /// Key for 'vm.compressor_eval_period_in_msecs'
    VmCompressor_eval_period_in_msecs,
    /// Key for 'vm.compressor_input_bytes'
    VmCompressor_input_bytes,
    /// Key for 'vm.compressor_is_active'
    VmCompressor_is_active,
    /// Key for 'vm.compressor_min_csegs_per_major_compaction'
    VmCompressor_min_csegs_per_major_compaction,
    /// Key for 'vm.compressor_mode'
    VmCompressor_mode,
    /// Key for 'vm.compressor_sample_max_in_msecs'
    VmCompressor_sample_max_in_msecs,
    /// Key for 'vm.compressor_sample_min_in_msecs'
    VmCompressor_sample_min_in_msecs,
    /// Key for 'vm.compressor_swapout_target_age'
    VmCompressor_swapout_target_age,
    /// Key for 'vm.compressor_thrashing_min_per_10msecs'
    VmCompressor_thrashing_min_per_10msecs,
    /// Key for 'vm.compressor_thrashing_threshold_per_10msecs'
    VmCompressor_thrashing_threshold_per_10msecs,
    /// Key for 'vm.compressor_timing_enabled'
    VmCompressor_timing_enabled,
    /// Key for 'vm.copied_on_read'
    VmCopied_on_read,
    /// Key for 'vm.corpse_footprint_count'
    VmCorpse_footprint_count,
    /// Key for 'vm.corpse_footprint_full'
    VmCorpse_footprint_full,
    /// Key for 'vm.corpse_footprint_no_buf'
    VmCorpse_footprint_no_buf,
    /// Key for 'vm.corpse_footprint_size_avg'
    VmCorpse_footprint_size_avg,
    /// Key for 'vm.corpse_footprint_size_max'
    VmCorpse_footprint_size_max,
    /// Key for 'vm.cs_all_vnodes'
    VmCs_all_vnodes,
    /// Key for 'vm.cs_blob_count'
    VmCs_blob_count,
    /// Key for 'vm.cs_blob_count_peak'
    VmCs_blob_count_peak,
    /// Key for 'vm.cs_blob_size'
    VmCs_blob_size,
    /// Key for 'vm.cs_blob_size_max'
    VmCs_blob_size_max,
    /// Key for 'vm.cs_blob_size_peak'
    VmCs_blob_size_peak,
    /// Key for 'vm.cs_debug'
    VmCs_debug,
    /// Key for 'vm.cs_debug_fail_on_unsigned_code'
    VmCs_debug_fail_on_unsigned_code,
    /// Key for 'vm.cs_debug_unsigned_exec_failures'
    VmCs_debug_unsigned_exec_failures,
    /// Key for 'vm.cs_debug_unsigned_mmap_failures'
    VmCs_debug_unsigned_mmap_failures,
    /// Key for 'vm.cs_enforcement_panic'
    VmCs_enforcement_panic,
    /// Key for 'vm.cs_force_hard'
    VmCs_force_hard,
    /// Key for 'vm.cs_force_kill'
    VmCs_force_kill,
    /// Key for 'vm.cs_library_validation'
    VmCs_library_validation,
    /// Key for 'vm.cs_process_enforcement'
    VmCs_process_enforcement,
    /// Key for 'vm.cs_system_enforcement'
    VmCs_system_enforcement,
    /// Key for 'vm.darkwake_mode'
    VmDarkwake_mode,
    /// Key for 'vm.free_shared'
    VmFree_shared,
    /// Key for 'vm.global_no_user_wire_amount'
    VmGlobal_no_user_wire_amount,
    /// Key for 'vm.global_user_wire_limit'
    VmGlobal_user_wire_limit,
    /// Key for 'vm.iopl_pages_tainted'
    VmIopl_pages_tainted,
    /// Key for 'vm.kern_lpage_count'
    VmKern_lpage_count,
    /// Key for 'vm.loadavg'
    VmLoadavg,
    /// Key for 'vm.lz4_compressed_bytes'
    VmLz4_compressed_bytes,
    /// Key for 'vm.lz4_compression_failures'
    VmLz4_compression_failures,
    /// Key for 'vm.lz4_compressions'
    VmLz4_compressions,
    /// Key for 'vm.lz4_decompressed_bytes'
    VmLz4_decompressed_bytes,
    /// Key for 'vm.lz4_decompressions'
    VmLz4_decompressions,
    /// Key for 'vm.lz4_max_failure_run_length'
    VmLz4_max_failure_run_length,
    /// Key for 'vm.lz4_max_failure_skips'
    VmLz4_max_failure_skips,
    /// Key for 'vm.lz4_max_preselects'
    VmLz4_max_preselects,
    /// Key for 'vm.lz4_profitable_bytes'
    VmLz4_profitable_bytes,
    /// Key for 'vm.lz4_run_continue_bytes'
    VmLz4_run_continue_bytes,
    /// Key for 'vm.lz4_run_preselection_threshold'
    VmLz4_run_preselection_threshold,
    /// Key for 'vm.lz4_threshold'
    VmLz4_threshold,
    /// Key for 'vm.lz4_wk_compression_delta'
    VmLz4_wk_compression_delta,
    /// Key for 'vm.lz4_wk_compression_negative_delta'
    VmLz4_wk_compression_negative_delta,
    /// Key for 'vm.madvise_free_debug'
    VmMadvise_free_debug,
    /// Key for 'vm.memory_pressure'
    VmMemory_pressure,
    /// Key for 'vm.page_busy_absent_skipped'
    VmPage_busy_absent_skipped,
    /// Key for 'vm.page_cleaned_count'
    VmPage_cleaned_count,
    /// Key for 'vm.page_free_count'
    VmPage_free_count,
    /// Key for 'vm.page_free_wanted'
    VmPage_free_wanted,
    /// Key for 'vm.page_pageable_external_count'
    VmPage_pageable_external_count,
    /// Key for 'vm.page_pageable_internal_count'
    VmPage_pageable_internal_count,
    /// Key for 'vm.page_purgeable_count'
    VmPage_purgeable_count,
    /// Key for 'vm.page_purgeable_wired_count'
    VmPage_purgeable_wired_count,
    /// Key for 'vm.page_reusable_count'
    VmPage_reusable_count,
    /// Key for 'vm.page_speculative_count'
    VmPage_speculative_count,
    /// Key for 'vm.pageout_freed_cleaned'
    VmPageout_freed_cleaned,
    /// Key for 'vm.pageout_freed_external'
    VmPageout_freed_external,
    /// Key for 'vm.pageout_freed_speculative'
    VmPageout_freed_speculative,
    /// Key for 'vm.pageout_inactive_clean'
    VmPageout_inactive_clean,
    /// Key for 'vm.pageout_inactive_dirty_external'
    VmPageout_inactive_dirty_external,
    /// Key for 'vm.pageout_inactive_dirty_internal'
    VmPageout_inactive_dirty_internal,
    /// Key for 'vm.pageout_inactive_used'
    VmPageout_inactive_used,
    /// Key for 'vm.pageout_speculative_clean'
    VmPageout_speculative_clean,
    /// Key for 'vm.pages'
    VmPages,
    /// Key for 'vm.pagesize'
    VmPagesize,
    /// Key for 'vm.partial_reusable_calls'
    VmPartial_reusable_calls,
    /// Key for 'vm.partial_reuse_calls'
    VmPartial_reuse_calls,
    /// Key for 'vm.prefault_nb_bailout'
    VmPrefault_nb_bailout,
    /// Key for 'vm.prefault_nb_pages'
    VmPrefault_nb_pages,
    /// Key for 'vm.protect_privileged_from_untrusted'
    VmProtect_privileged_from_untrusted,
    /// Key for 'vm.reusable_failure'
    VmReusable_failure,
    /// Key for 'vm.reusable_nonwritable'
    VmReusable_nonwritable,
    /// Key for 'vm.reusable_pages_shared'
    VmReusable_pages_shared,
    /// Key for 'vm.reusable_reclaimed'
    VmReusable_reclaimed,
    /// Key for 'vm.reusable_shared'
    VmReusable_shared,
    /// Key for 'vm.reusable_success'
    VmReusable_success,
    /// Key for 'vm.reuse_failure'
    VmReuse_failure,
    /// Key for 'vm.reuse_success'
    VmReuse_success,
    /// Key for 'vm.shared_region_pager_copied'
    VmShared_region_pager_copied,
    /// Key for 'vm.shared_region_pager_reclaimed'
    VmShared_region_pager_reclaimed,
    /// Key for 'vm.shared_region_pager_slid'
    VmShared_region_pager_slid,
    /// Key for 'vm.shared_region_pager_slid_error'
    VmShared_region_pager_slid_error,
    /// Key for 'vm.shared_region_persistence'
    VmShared_region_persistence,
    /// Key for 'vm.shared_region_trace_level'
    VmShared_region_trace_level,
    /// Key for 'vm.shared_region_unnest_logging'
    VmShared_region_unnest_logging,
    /// Key for 'vm.shared_region_version'
    VmShared_region_version,
    /// Key for 'vm.swapfileprefix'
    VmSwapfileprefix,
    /// Key for 'vm.swapusage'
    VmSwapusage,
    /// Key for 'vm.uc_decompressions'
    VmUc_decompressions,
    /// Key for 'vm.upl_pages_tainted'
    VmUpl_pages_tainted,
    /// Key for 'vm.user_wire_limit'
    VmUser_wire_limit,
    /// Key for 'vm.vm_clump_promote_threshold'
    VmVm_clump_promote_threshold,
    /// Key for 'vm.vm_copy_src_large'
    VmVm_copy_src_large,
    /// Key for 'vm.vm_copy_src_not_internal'
    VmVm_copy_src_not_internal,
    /// Key for 'vm.vm_copy_src_not_symmetric'
    VmVm_copy_src_not_symmetric,
    /// Key for 'vm.vm_create_upl_extra_cow'
    VmVm_create_upl_extra_cow,
    /// Key for 'vm.vm_create_upl_extra_cow_pages'
    VmVm_create_upl_extra_cow_pages,
    /// Key for 'vm.vm_create_upl_lookup_failure_copy'
    VmVm_create_upl_lookup_failure_copy,
    /// Key for 'vm.vm_create_upl_lookup_failure_write'
    VmVm_create_upl_lookup_failure_write,
    /// Key for 'vm.vm_debug_events'
    VmVm_debug_events,
    /// Key for 'vm.vm_do_collapse_compressor'
    VmVm_do_collapse_compressor,
    /// Key for 'vm.vm_do_collapse_compressor_pages'
    VmVm_do_collapse_compressor_pages,
    /// Key for 'vm.vm_do_collapse_terminate'
    VmVm_do_collapse_terminate,
    /// Key for 'vm.vm_do_collapse_terminate_failure'
    VmVm_do_collapse_terminate_failure,
    /// Key for 'vm.vm_max_kernel_address'
    VmVm_max_kernel_address,
    /// Key for 'vm.vm_min_kernel_address'
    VmVm_min_kernel_address,
    /// Key for 'vm.vm_page_background_count'
    VmVm_page_background_count,
    /// Key for 'vm.vm_page_background_exclude_external'
    VmVm_page_background_exclude_external,
    /// Key for 'vm.vm_page_background_external_count'
    VmVm_page_background_external_count,
    /// Key for 'vm.vm_page_background_internal_count'
    VmVm_page_background_internal_count,
    /// Key for 'vm.vm_page_background_mode'
    VmVm_page_background_mode,
    /// Key for 'vm.vm_page_background_promoted_count'
    VmVm_page_background_promoted_count,
    /// Key for 'vm.vm_page_background_target'
    VmVm_page_background_target,
    /// Key for 'vm.vm_page_external_count'
    VmVm_page_external_count,
    /// Key for 'vm.vm_page_filecache_min'
    VmVm_page_filecache_min,
    /// Key for 'vm.vm_page_free_target'
    VmVm_page_free_target,
    /// Key for 'vm.vm_page_xpmapped_min'
    VmVm_page_xpmapped_min,
    /// Key for 'vm.vm_pageout_considered_bq_external'
    VmVm_pageout_considered_bq_external,
    /// Key for 'vm.vm_pageout_considered_bq_internal'
    VmVm_pageout_considered_bq_internal,
    /// Key for 'vm.vm_pageout_rejected_bq_external'
    VmVm_pageout_rejected_bq_external,
    /// Key for 'vm.vm_pageout_rejected_bq_internal'
    VmVm_pageout_rejected_bq_internal,
    /// Key for 'vm.vm_ripe_target_age_in_secs'
    VmVm_ripe_target_age_in_secs,
    /// Key for 'vm.vm_should_cow_but_wired'
    VmVm_should_cow_but_wired,
    /// Key for 'vm.wk_catime'
    VmWk_catime,
    /// Key for 'vm.wk_compressed_bytes_exclusive'
    VmWk_compressed_bytes_exclusive,
    /// Key for 'vm.wk_compressed_bytes_total'
    VmWk_compressed_bytes_total,
    /// Key for 'vm.wk_compression_failures'
    VmWk_compression_failures,
    /// Key for 'vm.wk_compressions'
    VmWk_compressions,
    /// Key for 'vm.wk_compressions_exclusive'
    VmWk_compressions_exclusive,
    /// Key for 'vm.wk_datime'
    VmWk_datime,
    /// Key for 'vm.wk_decompressed_bytes'
    VmWk_decompressed_bytes,
    /// Key for 'vm.wk_decompressions'
    VmWk_decompressions,
    /// Key for 'vm.wk_mzv_compressions'
    VmWk_mzv_compressions,
    /// Key for 'vm.wk_sv_compressions'
    VmWk_sv_compressions,
    /// Key for 'vm.wk_sv_decompressions'
    VmWk_sv_decompressions,
    /// Key for 'vm.wkdm_reeval_threshold'
    VmWkdm_reeval_threshold,
    /// Key for 'vm.wkh_catime'
    VmWkh_catime,
    /// Key for 'vm.wkh_compressions'
    VmWkh_compressions,
    /// Key for 'vm.wkh_datime'
    VmWkh_datime,
    /// Key for 'vm.wkh_decompressions'
    VmWkh_decompressions,
    /// Key for 'vm.wks_catime'
    VmWks_catime,
    /// Key for 'vm.wks_compressed_bytes'
    VmWks_compressed_bytes,
    /// Key for 'vm.wks_compression_failures'
    VmWks_compression_failures,
    /// Key for 'vm.wks_compressions'
    VmWks_compressions,
    /// Key for 'vm.wks_datime'
    VmWks_datime,
    /// Key for 'vm.wks_decompressions'
    VmWks_decompressions,
    /// Key for 'vm.wks_sv_compressions'
    VmWks_sv_compressions,
}

impl SysctlKey {
    /// Returns the name of the key as it is named in
    /// the output of `$ sysctl -a`
    pub fn name(&self) -> &'static str {
        match self {
            SysctlKey::AuditSessionMember_clear_sflags_mask => "audit.session.member_clear_sflags_mask",
            SysctlKey::AuditSessionMember_set_sflags_mask => "audit.session.member_set_sflags_mask",
            SysctlKey::AuditSessionSuperuser_clear_sflags_mask => "audit.session.superuser_clear_sflags_mask",
            SysctlKey::AuditSessionSuperuser_set_sflags_mask => "audit.session.superuser_set_sflags_mask",
            SysctlKey::DebugAcpi_flags => "debug.acpi_flags",
            SysctlKey::DebugAcpi_layer => "debug.acpi_layer",
            SysctlKey::DebugAcpi_level => "debug.acpi_level",
            SysctlKey::DebugAgpmLogLevel => "debug.agpm.LogLevel",
            SysctlKey::DebugBatman => "debug.batman",
            SysctlKey::DebugBpf_bufsize => "debug.bpf_bufsize",
            SysctlKey::DebugBpf_debug => "debug.bpf_debug",
            SysctlKey::DebugBpf_maxbufsize => "debug.bpf_maxbufsize",
            SysctlKey::DebugBpf_maxdevices => "debug.bpf_maxdevices",
            SysctlKey::DebugBpf_wantpktap => "debug.bpf_wantpktap",
            SysctlKey::DebugBrcmfirewirelog => "debug.brcmfirewirelog",
            SysctlKey::DebugBrcmlinkdebug => "debug.brcmlinkdebug",
            SysctlKey::DebugBrcmlogging => "debug.brcmlogging",
            SysctlKey::DebugDarkwake => "debug.darkwake",
            SysctlKey::DebugIntelDtraceEnable => "debug.intel.dtraceEnable",
            SysctlKey::DebugIntelFlipCount => "debug.intel.flipCount",
            SysctlKey::DebugIntelGlobalUsageTotal_Busy_nSec => "debug.intel.GlobalUsageTotal_Busy_nSec",
            SysctlKey::DebugIntelGlobalUsageTotal_nSec => "debug.intel.GlobalUsageTotal_nSec",
            SysctlKey::DebugIntelGpuUsageEnables => "debug.intel.gpuUsageEnables",
            SysctlKey::DebugIntelGpuUsageEnablesCheck => "debug.intel.gpuUsageEnablesCheck",
            SysctlKey::DebugIntelGraphicsTracePointEnable => "debug.intel.graphicsTracePointEnable",
            SysctlKey::DebugIntelIGInterruptControl => "debug.intel.IGInterruptControl",
            SysctlKey::DebugIntelKdctlVersion => "debug.intel.kdctlVersion",
            SysctlKey::DebugIntelMSecCalcGPUBusy => "debug.intel.mSecCalcGPUBusy",
            SysctlKey::DebugIntelOaEnable => "debug.intel.oaEnable",
            SysctlKey::DebugIntelPerfEventEnable => "debug.intel.perfEventEnable",
            SysctlKey::DebugIntelRingBlitUsage => "debug.intel.ringBlitUsage",
            SysctlKey::DebugIntelRingBlit_nSec => "debug.intel.ringBlit_nSec",
            SysctlKey::DebugIntelRingMainUsage => "debug.intel.ringMainUsage",
            SysctlKey::DebugIntelRingMain_nSec => "debug.intel.ringMain_nSec",
            SysctlKey::DebugIntelRingMediaUsage => "debug.intel.ringMediaUsage",
            SysctlKey::DebugIntelRingMedia_nSec => "debug.intel.ringMedia_nSec",
            SysctlKey::DebugIntelRingOnSample => "debug.intel.ringOnSample",
            SysctlKey::DebugIntelRingTakeSample => "debug.intel.ringTakeSample",
            SysctlKey::DebugIntelRingVEBoxUsage => "debug.intel.ringVEBoxUsage",
            SysctlKey::DebugIntelRingVEBox_nSec => "debug.intel.ringVEBox_nSec",
            SysctlKey::DebugIntelSchedEnableThrottleOverride => "debug.intel.schedEnableThrottleOverride",
            SysctlKey::DebugIntelSchedPriCreditsHigh => "debug.intel.schedPriCreditsHigh",
            SysctlKey::DebugIntelSchedPriCreditsLow => "debug.intel.schedPriCreditsLow",
            SysctlKey::DebugIntelSchedPriCreditsNormal => "debug.intel.schedPriCreditsNormal",
            SysctlKey::DebugIntelSchedPriCreditsNormalHigh => "debug.intel.schedPriCreditsNormalHigh",
            SysctlKey::DebugIntelSchedPriElevatePID => "debug.intel.schedPriElevatePID",
            SysctlKey::DebugIntelSchedPriPreemption => "debug.intel.schedPriPreemption",
            SysctlKey::DebugIntelSchedThrottleHighPriByVal => "debug.intel.schedThrottleHighPriByVal",
            SysctlKey::DebugIntelSchedThrottleLowPriByVal => "debug.intel.schedThrottleLowPriByVal",
            SysctlKey::DebugIntelSchedThrottleNormalHighPriByVal => "debug.intel.schedThrottleNormalHighPriByVal",
            SysctlKey::DebugIntelSchedThrottleNormalPriByVal => "debug.intel.schedThrottleNormalPriByVal",
            SysctlKey::DebugIntelSwapCount => "debug.intel.swapCount",
            SysctlKey::DebugIntelTelemetryAltConfig => "debug.intel.telemetryAltConfig",
            SysctlKey::DebugIntelTelemetryConfig => "debug.intel.telemetryConfig",
            SysctlKey::DebugIntelTelemetryMode => "debug.intel.telemetryMode",
            SysctlKey::DebugIntelTelemetryNumFrame => "debug.intel.telemetryNumFrame",
            SysctlKey::DebugIntelTelemetrySampleLocations => "debug.intel.telemetrySampleLocations",
            SysctlKey::DebugIntelTelemetrySpot1 => "debug.intel.telemetrySpot1",
            SysctlKey::DebugIntelTelemetryStartFrame => "debug.intel.telemetryStartFrame",
            SysctlKey::DebugIntelTelemetryStatPasses => "debug.intel.telemetryStatPasses",
            SysctlKey::DebugIntelTelemetryStopFrame => "debug.intel.telemetryStopFrame",
            SysctlKey::DebugIntelTelemetryTestCase => "debug.intel.telemetryTestCase",
            SysctlKey::DebugIntelTelemetryUsageReportmSec => "debug.intel.telemetryUsageReportmSec",
            SysctlKey::DebugIntelTelemetryVersion => "debug.intel.telemetryVersion",
            SysctlKey::DebugIntelTemp0 => "debug.intel.temp0",
            SysctlKey::DebugIntelTemp1 => "debug.intel.temp1",
            SysctlKey::DebugIntelTemp2 => "debug.intel.temp2",
            SysctlKey::DebugIntelTemp3 => "debug.intel.temp3",
            SysctlKey::DebugIntelTemp4 => "debug.intel.temp4",
            SysctlKey::DebugIntelfbEUCount => "debug.intelfb.EUCount",
            SysctlKey::DebugIntelfbFLastRequestedPState => "debug.intelfb.fLastRequestedPState",
            SysctlKey::DebugIntelfbFakeType2Dongle => "debug.intelfb.FakeType2Dongle",
            SysctlKey::DebugIntelfbForceSlicesGTx => "debug.intelfb.forceSlicesGTx",
            SysctlKey::DebugIntelfbGraphicsTracePointEnable => "debug.intelfb.graphicsTracePointEnable",
            SysctlKey::DebugIntelfbIGInterruptControl => "debug.intelfb.IGInterruptControl",
            SysctlKey::DebugIntelfbSliceInfo => "debug.intelfb.sliceInfo",
            SysctlKey::DebugIntelfbTemp0 => "debug.intelfb.temp0",
            SysctlKey::DebugIntelfbTemp1 => "debug.intelfb.temp1",
            SysctlKey::DebugIntelfbTemp2 => "debug.intelfb.temp2",
            SysctlKey::DebugIntelfbTemp3 => "debug.intelfb.temp3",
            SysctlKey::DebugIntelfbTemp4 => "debug.intelfb.temp4",
            SysctlKey::DebugIntelfbTestCase => "debug.intelfb.testCase",
            SysctlKey::DebugIokit => "debug.iokit",
            SysctlKey::DebugIoppf => "debug.ioppf",
            SysctlKey::DebugIotrace => "debug.iotrace",
            SysctlKey::DebugKextlog => "debug.kextlog",
            SysctlKey::DebugLowpri_throttle_enabled => "debug.lowpri_throttle_enabled",
            SysctlKey::DebugLowpri_throttle_max_iosize => "debug.lowpri_throttle_max_iosize",
            SysctlKey::DebugLowpri_throttle_tier1_io_period_msecs => "debug.lowpri_throttle_tier1_io_period_msecs",
            SysctlKey::DebugLowpri_throttle_tier1_io_period_ssd_msecs => "debug.lowpri_throttle_tier1_io_period_ssd_msecs",
            SysctlKey::DebugLowpri_throttle_tier1_window_msecs => "debug.lowpri_throttle_tier1_window_msecs",
            SysctlKey::DebugLowpri_throttle_tier2_io_period_msecs => "debug.lowpri_throttle_tier2_io_period_msecs",
            SysctlKey::DebugLowpri_throttle_tier2_io_period_ssd_msecs => "debug.lowpri_throttle_tier2_io_period_ssd_msecs",
            SysctlKey::DebugLowpri_throttle_tier2_window_msecs => "debug.lowpri_throttle_tier2_window_msecs",
            SysctlKey::DebugLowpri_throttle_tier3_io_period_msecs => "debug.lowpri_throttle_tier3_io_period_msecs",
            SysctlKey::DebugLowpri_throttle_tier3_io_period_ssd_msecs => "debug.lowpri_throttle_tier3_io_period_ssd_msecs",
            SysctlKey::DebugLowpri_throttle_tier3_window_msecs => "debug.lowpri_throttle_tier3_window_msecs",
            SysctlKey::DebugNoidle => "debug.noidle",
            SysctlKey::DebugSched => "debug.sched",
            SysctlKey::DebugSwd_panic => "debug.swd_panic",
            SysctlKey::DebugSwd_sleep_timeout => "debug.swd_sleep_timeout",
            SysctlKey::DebugSwd_timeout => "debug.swd_timeout",
            SysctlKey::DebugSwd_wake_timeout => "debug.swd_wake_timeout",
            SysctlKey::DebugToggle_address_reuse => "debug.toggle_address_reuse",
            SysctlKey::DebugWlan_ll_debug => "debug.wlan_ll_debug",
            SysctlKey::HwActivecpu => "hw.activecpu",
            SysctlKey::HwBusfrequency => "hw.busfrequency",
            SysctlKey::HwBusfrequency_max => "hw.busfrequency_max",
            SysctlKey::HwBusfrequency_min => "hw.busfrequency_min",
            SysctlKey::HwByteorder => "hw.byteorder",
            SysctlKey::HwCacheconfig => "hw.cacheconfig",
            SysctlKey::HwCachelinesize => "hw.cachelinesize",
            SysctlKey::HwCachesize => "hw.cachesize",
            SysctlKey::HwCpu64bit_capable => "hw.cpu64bit_capable",
            SysctlKey::HwCpufamily => "hw.cpufamily",
            SysctlKey::HwCpufrequency => "hw.cpufrequency",
            SysctlKey::HwCpufrequency_max => "hw.cpufrequency_max",
            SysctlKey::HwCpufrequency_min => "hw.cpufrequency_min",
            SysctlKey::HwCpusubtype => "hw.cpusubtype",
            SysctlKey::HwCputhreadtype => "hw.cputhreadtype",
            SysctlKey::HwCputype => "hw.cputype",
            SysctlKey::HwL1dcachesize => "hw.l1dcachesize",
            SysctlKey::HwL1icachesize => "hw.l1icachesize",
            SysctlKey::HwL2cachesize => "hw.l2cachesize",
            SysctlKey::HwL3cachesize => "hw.l3cachesize",
            SysctlKey::HwLogicalcpu => "hw.logicalcpu",
            SysctlKey::HwLogicalcpu_max => "hw.logicalcpu_max",
            SysctlKey::HwMemsize => "hw.memsize",
            SysctlKey::HwNcpu => "hw.ncpu",
            SysctlKey::HwOptionalAdx => "hw.optional.adx",
            SysctlKey::HwOptionalAes => "hw.optional.aes",
            SysctlKey::HwOptionalAvx1_0 => "hw.optional.avx1_0",
            SysctlKey::HwOptionalAvx2_0 => "hw.optional.avx2_0",
            SysctlKey::HwOptionalAvx512bw => "hw.optional.avx512bw",
            SysctlKey::HwOptionalAvx512cd => "hw.optional.avx512cd",
            SysctlKey::HwOptionalAvx512dq => "hw.optional.avx512dq",
            SysctlKey::HwOptionalAvx512f => "hw.optional.avx512f",
            SysctlKey::HwOptionalAvx512ifma => "hw.optional.avx512ifma",
            SysctlKey::HwOptionalAvx512vbmi => "hw.optional.avx512vbmi",
            SysctlKey::HwOptionalAvx512vl => "hw.optional.avx512vl",
            SysctlKey::HwOptionalBmi1 => "hw.optional.bmi1",
            SysctlKey::HwOptionalBmi2 => "hw.optional.bmi2",
            SysctlKey::HwOptionalEnfstrg => "hw.optional.enfstrg",
            SysctlKey::HwOptionalF16c => "hw.optional.f16c",
            SysctlKey::HwOptionalFloatingpoint => "hw.optional.floatingpoint",
            SysctlKey::HwOptionalFma => "hw.optional.fma",
            SysctlKey::HwOptionalHle => "hw.optional.hle",
            SysctlKey::HwOptionalMmx => "hw.optional.mmx",
            SysctlKey::HwOptionalMpx => "hw.optional.mpx",
            SysctlKey::HwOptionalRdrand => "hw.optional.rdrand",
            SysctlKey::HwOptionalRtm => "hw.optional.rtm",
            SysctlKey::HwOptionalSgx => "hw.optional.sgx",
            SysctlKey::HwOptionalSse => "hw.optional.sse",
            SysctlKey::HwOptionalSse2 => "hw.optional.sse2",
            SysctlKey::HwOptionalSse3 => "hw.optional.sse3",
            SysctlKey::HwOptionalSse4_1 => "hw.optional.sse4_1",
            SysctlKey::HwOptionalSse4_2 => "hw.optional.sse4_2",
            SysctlKey::HwOptionalSupplementalsse3 => "hw.optional.supplementalsse3",
            SysctlKey::HwOptionalX86_64 => "hw.optional.x86_64",
            SysctlKey::HwPackages => "hw.packages",
            SysctlKey::HwPagesize => "hw.pagesize",
            SysctlKey::HwPagesize32 => "hw.pagesize32",
            SysctlKey::HwPhysicalcpu => "hw.physicalcpu",
            SysctlKey::HwPhysicalcpu_max => "hw.physicalcpu_max",
            SysctlKey::HwTargettype => "hw.targettype",
            SysctlKey::HwTbfrequency => "hw.tbfrequency",
            SysctlKey::KernAffinity_sets_enabled => "kern.affinity_sets_enabled",
            SysctlKey::KernAffinity_sets_mapping => "kern.affinity_sets_mapping",
            SysctlKey::KernAiomax => "kern.aiomax",
            SysctlKey::KernAioprocmax => "kern.aioprocmax",
            SysctlKey::KernAiothreads => "kern.aiothreads",
            SysctlKey::KernAotmode => "kern.aotmode",
            SysctlKey::KernAotmodebits => "kern.aotmodebits",
            SysctlKey::KernArgmax => "kern.argmax",
            SysctlKey::KernBootargs => "kern.bootargs",
            SysctlKey::KernBootsessionuuid => "kern.bootsessionuuid",
            SysctlKey::KernBootsignature => "kern.bootsignature",
            SysctlKey::KernBoottime => "kern.boottime",
            SysctlKey::KernCheck_openevt => "kern.check_openevt",
            SysctlKey::KernClockrate => "kern.clockrate",
            SysctlKey::KernConsoleoptions => "kern.consoleoptions",
            SysctlKey::KernCoredump => "kern.coredump",
            SysctlKey::KernCorefile => "kern.corefile",
            SysctlKey::KernDelayterm => "kern.delayterm",
            SysctlKey::KernDs_supgroups_supported => "kern.ds_supgroups_supported",
            SysctlKey::KernDtraceBuffer_memory_inuse => "kern.dtrace.buffer_memory_inuse",
            SysctlKey::KernDtraceBuffer_memory_maxsize => "kern.dtrace.buffer_memory_maxsize",
            SysctlKey::KernDtraceDifo_maxsize => "kern.dtrace.difo_maxsize",
            SysctlKey::KernDtraceDof_maxsize => "kern.dtrace.dof_maxsize",
            SysctlKey::KernDtraceDof_mode => "kern.dtrace.dof_mode",
            SysctlKey::KernDtraceErr_verbose => "kern.dtrace.err_verbose",
            SysctlKey::KernDtraceGlobal_maxsize => "kern.dtrace.global_maxsize",
            SysctlKey::KernDtraceIgnore_fbt_blacklist => "kern.dtrace.ignore_fbt_blacklist",
            SysctlKey::KernDtraceProvide_private_probes => "kern.dtrace.provide_private_probes",
            SysctlKey::KernEventhandlerDebug => "kern.eventhandler.debug",
            SysctlKey::KernFlush_cache_on_write => "kern.flush_cache_on_write",
            SysctlKey::KernHibernatefile => "kern.hibernatefile",
            SysctlKey::KernHibernategraphicsready => "kern.hibernategraphicsready",
            SysctlKey::KernHibernatehidready => "kern.hibernatehidready",
            SysctlKey::KernHibernatelockscreenready => "kern.hibernatelockscreenready",
            SysctlKey::KernHibernatemode => "kern.hibernatemode",
            SysctlKey::KernHibernatewakenotification => "kern.hibernatewakenotification",
            SysctlKey::KernHostid => "kern.hostid",
            SysctlKey::KernHostname => "kern.hostname",
            SysctlKey::KernHvVmx_mitigations => "kern.hv.vmx_mitigations",
            SysctlKey::KernHvVmx_supported_mitigations => "kern.hv.vmx_supported_mitigations",
            SysctlKey::KernHv_support => "kern.hv_support",
            SysctlKey::KernInterrupt_timer_coalescing_enabled => "kern.interrupt_timer_coalescing_enabled",
            SysctlKey::KernIokittest => "kern.iokittest",
            SysctlKey::KernIossupportversion => "kern.iossupportversion",
            SysctlKey::KernIpcExtbkidlercvhiwat => "kern.ipc.extbkidlercvhiwat",
            SysctlKey::KernIpcExtbkidletime => "kern.ipc.extbkidletime",
            SysctlKey::KernIpcIo_policyLog => "kern.ipc.io_policy.log",
            SysctlKey::KernIpcIo_policyUuid => "kern.ipc.io_policy.uuid",
            SysctlKey::KernIpcMaxextbkidleperproc => "kern.ipc.maxextbkidleperproc",
            SysctlKey::KernIpcMaxrecvmsgx => "kern.ipc.maxrecvmsgx",
            SysctlKey::KernIpcMaxsendmsgx => "kern.ipc.maxsendmsgx",
            SysctlKey::KernIpcMaxsockbuf => "kern.ipc.maxsockbuf",
            SysctlKey::KernIpcMb_drain_force => "kern.ipc.mb_drain_force",
            SysctlKey::KernIpcMb_drain_maxint => "kern.ipc.mb_drain_maxint",
            SysctlKey::KernIpcMb_normalized => "kern.ipc.mb_normalized",
            SysctlKey::KernIpcMb_watchdog => "kern.ipc.mb_watchdog",
            SysctlKey::KernIpcMleak_sample_factor => "kern.ipc.mleak_sample_factor",
            SysctlKey::KernIpcNjcl => "kern.ipc.njcl",
            SysctlKey::KernIpcNjclbytes => "kern.ipc.njclbytes",
            SysctlKey::KernIpcNmbclusters => "kern.ipc.nmbclusters",
            SysctlKey::KernIpcSbmb_cnt => "kern.ipc.sbmb_cnt",
            SysctlKey::KernIpcSbmb_cnt_floor => "kern.ipc.sbmb_cnt_floor",
            SysctlKey::KernIpcSbmb_cnt_peak => "kern.ipc.sbmb_cnt_peak",
            SysctlKey::KernIpcSbmb_limreached => "kern.ipc.sbmb_limreached",
            SysctlKey::KernIpcSockbuf_waste_factor => "kern.ipc.sockbuf_waste_factor",
            SysctlKey::KernIpcSocket_debug => "kern.ipc.socket_debug",
            SysctlKey::KernIpcSodefunct_calls => "kern.ipc.sodefunct_calls",
            SysctlKey::KernIpcSodefunctlog => "kern.ipc.sodefunctlog",
            SysctlKey::KernIpcSomaxconn => "kern.ipc.somaxconn",
            SysctlKey::KernIpcSoqlencomp => "kern.ipc.soqlencomp",
            SysctlKey::KernIpcSoqlimitcompat => "kern.ipc.soqlimitcompat",
            SysctlKey::KernIpcSorecvmincopy => "kern.ipc.sorecvmincopy",
            SysctlKey::KernIpcSoreserveheadroom => "kern.ipc.soreserveheadroom",
            SysctlKey::KernIpcSorestrictrecv => "kern.ipc.sorestrictrecv",
            SysctlKey::KernIpcSorestrictsend => "kern.ipc.sorestrictsend",
            SysctlKey::KernIpcSosendbigcl_ignore_capab => "kern.ipc.sosendbigcl_ignore_capab",
            SysctlKey::KernIpcSosendjcl => "kern.ipc.sosendjcl",
            SysctlKey::KernIpcSosendjcl_ignore_capab => "kern.ipc.sosendjcl_ignore_capab",
            SysctlKey::KernIpcSosendminchain => "kern.ipc.sosendminchain",
            SysctlKey::KernIpcSotcdb => "kern.ipc.sotcdb",
            SysctlKey::KernIpcSothrottlelog => "kern.ipc.sothrottlelog",
            SysctlKey::KernIpcThrottle_best_effort => "kern.ipc.throttle_best_effort",
            SysctlKey::KernIpc_portbt => "kern.ipc_portbt",
            SysctlKey::KernIpc_voucher_trace_contents => "kern.ipc_voucher_trace_contents",
            SysctlKey::KernJetsam_aging_policy => "kern.jetsam_aging_policy",
            SysctlKey::KernJob_control => "kern.job_control",
            SysctlKey::KernKdbgDebug => "kern.kdbg.debug",
            SysctlKey::KernKdbgExperimental_continuous => "kern.kdbg.experimental_continuous",
            SysctlKey::KernKdbgOldest_time => "kern.kdbg.oldest_time",
            SysctlKey::KernKern_feature_overrides => "kern.kern_feature_overrides",
            SysctlKey::KernKernelcacheuuid => "kern.kernelcacheuuid",
            SysctlKey::KernMaxfiles => "kern.maxfiles",
            SysctlKey::KernMaxfilesperproc => "kern.maxfilesperproc",
            SysctlKey::KernMaxnbuf => "kern.maxnbuf",
            SysctlKey::KernMaxproc => "kern.maxproc",
            SysctlKey::KernMaxprocperuid => "kern.maxprocperuid",
            SysctlKey::KernMaxvnodes => "kern.maxvnodes",
            SysctlKey::KernMemorystatus_apps_idle_delay_time => "kern.memorystatus_apps_idle_delay_time",
            SysctlKey::KernMemorystatus_purge_on_critical => "kern.memorystatus_purge_on_critical",
            SysctlKey::KernMemorystatus_purge_on_urgent => "kern.memorystatus_purge_on_urgent",
            SysctlKey::KernMemorystatus_purge_on_warning => "kern.memorystatus_purge_on_warning",
            SysctlKey::KernMemorystatus_sysprocs_idle_delay_time => "kern.memorystatus_sysprocs_idle_delay_time",
            SysctlKey::KernMinimalboot => "kern.minimalboot",
            SysctlKey::KernMonotonicPmis => "kern.monotonic.pmis",
            SysctlKey::KernMonotonicRetrograde_updates => "kern.monotonic.retrograde_updates",
            SysctlKey::KernMonotonicSupported => "kern.monotonic.supported",
            SysctlKey::KernMonotonicTask_thread_counting => "kern.monotonic.task_thread_counting",
            SysctlKey::KernMsgbuf => "kern.msgbuf",
            SysctlKey::KernNamecache_disabled => "kern.namecache_disabled",
            SysctlKey::KernNbuf => "kern.nbuf",
            SysctlKey::KernNetboot => "kern.netboot",
            SysctlKey::KernNgroups => "kern.ngroups",
            SysctlKey::KernNisdomainname => "kern.nisdomainname",
            SysctlKey::KernNum_files => "kern.num_files",
            SysctlKey::KernNum_recycledvnodes => "kern.num_recycledvnodes",
            SysctlKey::KernNum_tasks => "kern.num_tasks",
            SysctlKey::KernNum_taskthreads => "kern.num_taskthreads",
            SysctlKey::KernNum_threads => "kern.num_threads",
            SysctlKey::KernNum_vnodes => "kern.num_vnodes",
            SysctlKey::KernOsproductversion => "kern.osproductversion",
            SysctlKey::KernOsrelease => "kern.osrelease",
            SysctlKey::KernOsrevision => "kern.osrevision",
            SysctlKey::KernOstype => "kern.ostype",
            SysctlKey::KernOsversion => "kern.osversion",
            SysctlKey::KernPmtimeout => "kern.pmtimeout",
            SysctlKey::KernPosix1version => "kern.posix1version",
            SysctlKey::KernPosixSemMax => "kern.posix.sem.max",
            SysctlKey::KernPreheat_max_bytes => "kern.preheat_max_bytes",
            SysctlKey::KernPreheat_min_bytes => "kern.preheat_min_bytes",
            SysctlKey::KernProcname => "kern.procname",
            SysctlKey::KernProgressmeter => "kern.progressmeter",
            SysctlKey::KernProgressmeterenable => "kern.progressmeterenable",
            SysctlKey::KernPthread_mutex_default_policy => "kern.pthread_mutex_default_policy",
            SysctlKey::KernRage_vnode => "kern.rage_vnode",
            SysctlKey::KernSafeboot => "kern.safeboot",
            SysctlKey::KernSaved_ids => "kern.saved_ids",
            SysctlKey::KernSched => "kern.sched",
            SysctlKey::KernSched_allow_NO_SMT_threads => "kern.sched_allow_NO_SMT_threads",
            SysctlKey::KernSched_enable_smt => "kern.sched_enable_smt",
            SysctlKey::KernSecure_kernel => "kern.secure_kernel",
            SysctlKey::KernSecurelevel => "kern.securelevel",
            SysctlKey::KernShreg_private => "kern.shreg_private",
            SysctlKey::KernSingleuser => "kern.singleuser",
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmFrag_count => "kern.skywalk.flowswitch.awdl0.ipfm.frag_count",
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmFrag_limit => "kern.skywalk.flowswitch.awdl0.ipfm.frag_limit",
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmQueue_count => "kern.skywalk.flowswitch.awdl0.ipfm.queue_count",
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmQueue_limit => "kern.skywalk.flowswitch.awdl0.ipfm.queue_limit",
            SysctlKey::KernSkywalkFlowswitchEn0IpfmFrag_count => "kern.skywalk.flowswitch.en0.ipfm.frag_count",
            SysctlKey::KernSkywalkFlowswitchEn0IpfmFrag_limit => "kern.skywalk.flowswitch.en0.ipfm.frag_limit",
            SysctlKey::KernSkywalkFlowswitchEn0IpfmQueue_count => "kern.skywalk.flowswitch.en0.ipfm.queue_count",
            SysctlKey::KernSkywalkFlowswitchEn0IpfmQueue_limit => "kern.skywalk.flowswitch.en0.ipfm.queue_limit",
            SysctlKey::KernSkywalkFlowswitchEn1IpfmFrag_count => "kern.skywalk.flowswitch.en1.ipfm.frag_count",
            SysctlKey::KernSkywalkFlowswitchEn1IpfmFrag_limit => "kern.skywalk.flowswitch.en1.ipfm.frag_limit",
            SysctlKey::KernSkywalkFlowswitchEn1IpfmQueue_count => "kern.skywalk.flowswitch.en1.ipfm.queue_count",
            SysctlKey::KernSkywalkFlowswitchEn1IpfmQueue_limit => "kern.skywalk.flowswitch.en1.ipfm.queue_limit",
            SysctlKey::KernSkywalkFlowswitchEn2IpfmFrag_count => "kern.skywalk.flowswitch.en2.ipfm.frag_count",
            SysctlKey::KernSkywalkFlowswitchEn2IpfmFrag_limit => "kern.skywalk.flowswitch.en2.ipfm.frag_limit",
            SysctlKey::KernSkywalkFlowswitchEn2IpfmQueue_count => "kern.skywalk.flowswitch.en2.ipfm.queue_count",
            SysctlKey::KernSkywalkFlowswitchEn2IpfmQueue_limit => "kern.skywalk.flowswitch.en2.ipfm.queue_limit",
            SysctlKey::KernSkywalkFlowswitchFlow_route_expire => "kern.skywalk.flowswitch.flow_route_expire",
            SysctlKey::KernSkywalkFlowswitchIp_reass => "kern.skywalk.flowswitch.ip_reass",
            SysctlKey::KernSkywalkFlowswitchIpfm_frag_ttl => "kern.skywalk.flowswitch.ipfm_frag_ttl",
            SysctlKey::KernSkywalkFlowswitchIpfm_timeout_tcall_ival => "kern.skywalk.flowswitch.ipfm_timeout_tcall_ival",
            SysctlKey::KernSkywalkNetifDefault_drop => "kern.skywalk.netif.default_drop",
            SysctlKey::KernSkywalkRing_stat_enable => "kern.skywalk.ring_stat_enable",
            SysctlKey::KernSleep_abs_time => "kern.sleep_abs_time",
            SysctlKey::KernSleeptime => "kern.sleeptime",
            SysctlKey::KernSlide => "kern.slide",
            SysctlKey::KernSpeculative_prefetch_max => "kern.speculative_prefetch_max",
            SysctlKey::KernSpeculative_prefetch_max_iosize => "kern.speculative_prefetch_max_iosize",
            SysctlKey::KernSpeculative_reads_disabled => "kern.speculative_reads_disabled",
            SysctlKey::KernStack_depth_max => "kern.stack_depth_max",
            SysctlKey::KernStack_size => "kern.stack_size",
            SysctlKey::KernSugid_coredump => "kern.sugid_coredump",
            SysctlKey::KernSugid_scripts => "kern.sugid_scripts",
            SysctlKey::KernSysvSemmni => "kern.sysv.semmni",
            SysctlKey::KernSysvSemmns => "kern.sysv.semmns",
            SysctlKey::KernSysvSemmnu => "kern.sysv.semmnu",
            SysctlKey::KernSysvSemmsl => "kern.sysv.semmsl",
            SysctlKey::KernSysvSemume => "kern.sysv.semume",
            SysctlKey::KernSysvShmall => "kern.sysv.shmall",
            SysctlKey::KernSysvShmmax => "kern.sysv.shmmax",
            SysctlKey::KernSysvShmmin => "kern.sysv.shmmin",
            SysctlKey::KernSysvShmmni => "kern.sysv.shmmni",
            SysctlKey::KernSysvShmseg => "kern.sysv.shmseg",
            SysctlKey::KernTask_exc_guard_default => "kern.task_exc_guard_default",
            SysctlKey::KernTfpPolicy => "kern.tfp.policy",
            SysctlKey::KernThread_groups_supported => "kern.thread_groups_supported",
            SysctlKey::KernThreadname => "kern.threadname",
            SysctlKey::KernTimerCoalescing_enabled => "kern.timer.coalescing_enabled",
            SysctlKey::KernTimerDeadline_tracking_bin_1 => "kern.timer.deadline_tracking_bin_1",
            SysctlKey::KernTimerDeadline_tracking_bin_2 => "kern.timer.deadline_tracking_bin_2",
            SysctlKey::KernTimerLongtermQlen => "kern.timer.longterm.qlen",
            SysctlKey::KernTimerLongtermScan_interval => "kern.timer.longterm.scan_interval",
            SysctlKey::KernTimerLongtermScan_limit => "kern.timer.longterm.scan_limit",
            SysctlKey::KernTimerLongtermScan_pauses => "kern.timer.longterm.scan_pauses",
            SysctlKey::KernTimerLongtermThreshold => "kern.timer.longterm.threshold",
            SysctlKey::KernTimer_coalesce_bg_ns_max => "kern.timer_coalesce_bg_ns_max",
            SysctlKey::KernTimer_coalesce_bg_scale => "kern.timer_coalesce_bg_scale",
            SysctlKey::KernTimer_coalesce_fp_ns_max => "kern.timer_coalesce_fp_ns_max",
            SysctlKey::KernTimer_coalesce_fp_scale => "kern.timer_coalesce_fp_scale",
            SysctlKey::KernTimer_coalesce_idle_entry_hard_deadline_max => "kern.timer_coalesce_idle_entry_hard_deadline_max",
            SysctlKey::KernTimer_coalesce_kt_ns_max => "kern.timer_coalesce_kt_ns_max",
            SysctlKey::KernTimer_coalesce_kt_scale => "kern.timer_coalesce_kt_scale",
            SysctlKey::KernTimer_coalesce_tier0_ns_max => "kern.timer_coalesce_tier0_ns_max",
            SysctlKey::KernTimer_coalesce_tier0_scale => "kern.timer_coalesce_tier0_scale",
            SysctlKey::KernTimer_coalesce_tier1_ns_max => "kern.timer_coalesce_tier1_ns_max",
            SysctlKey::KernTimer_coalesce_tier1_scale => "kern.timer_coalesce_tier1_scale",
            SysctlKey::KernTimer_coalesce_tier2_ns_max => "kern.timer_coalesce_tier2_ns_max",
            SysctlKey::KernTimer_coalesce_tier2_scale => "kern.timer_coalesce_tier2_scale",
            SysctlKey::KernTimer_coalesce_tier3_ns_max => "kern.timer_coalesce_tier3_ns_max",
            SysctlKey::KernTimer_coalesce_tier3_scale => "kern.timer_coalesce_tier3_scale",
            SysctlKey::KernTimer_coalesce_tier4_ns_max => "kern.timer_coalesce_tier4_ns_max",
            SysctlKey::KernTimer_coalesce_tier4_scale => "kern.timer_coalesce_tier4_scale",
            SysctlKey::KernTimer_coalesce_tier5_ns_max => "kern.timer_coalesce_tier5_ns_max",
            SysctlKey::KernTimer_coalesce_tier5_scale => "kern.timer_coalesce_tier5_scale",
            SysctlKey::KernTimer_coalesce_ts_ns_max => "kern.timer_coalesce_ts_ns_max",
            SysctlKey::KernTimer_coalesce_ts_scale => "kern.timer_coalesce_ts_scale",
            SysctlKey::KernTimer_resort_threshold_ns => "kern.timer_resort_threshold_ns",
            SysctlKey::KernTtyPtmx_max => "kern.tty.ptmx_max",
            SysctlKey::KernUlock_adaptive_spin_usecs => "kern.ulock_adaptive_spin_usecs",
            SysctlKey::KernUseractive_abs_time => "kern.useractive_abs_time",
            SysctlKey::KernUserinactive_abs_time => "kern.userinactive_abs_time",
            SysctlKey::KernUsrstack => "kern.usrstack",
            SysctlKey::KernUsrstack64 => "kern.usrstack64",
            SysctlKey::KernUuid => "kern.uuid",
            SysctlKey::KernVersion => "kern.version",
            SysctlKey::KernVfsnspace => "kern.vfsnspace",
            SysctlKey::KernVm_max_batch => "kern.vm_max_batch",
            SysctlKey::KernVm_max_delayed_work_limit => "kern.vm_max_delayed_work_limit",
            SysctlKey::KernVm_page_free_min => "kern.vm_page_free_min",
            SysctlKey::KernVm_page_free_reserved => "kern.vm_page_free_reserved",
            SysctlKey::KernVm_page_free_target => "kern.vm_page_free_target",
            SysctlKey::KernVm_page_speculative_percentage => "kern.vm_page_speculative_percentage",
            SysctlKey::KernVm_page_speculative_q_age_ms => "kern.vm_page_speculative_q_age_ms",
            SysctlKey::KernWake_abs_time => "kern.wake_abs_time",
            SysctlKey::KernWakereason => "kern.wakereason",
            SysctlKey::KernWaketime => "kern.waketime",
            SysctlKey::KernWillshutdown => "kern.willshutdown",
            SysctlKey::KernWq_max_constrained_threads => "kern.wq_max_constrained_threads",
            SysctlKey::KernWq_max_threads => "kern.wq_max_threads",
            SysctlKey::KernWq_max_timer_interval_usecs => "kern.wq_max_timer_interval_usecs",
            SysctlKey::KernWq_reduce_pool_window_usecs => "kern.wq_reduce_pool_window_usecs",
            SysctlKey::KernWq_stalled_window_usecs => "kern.wq_stalled_window_usecs",
            SysctlKey::KernZleakActive => "kern.zleak.active",
            SysctlKey::KernZleakGlobal_threshold => "kern.zleak.global_threshold",
            SysctlKey::KernZleakMax_zonemap_size => "kern.zleak.max_zonemap_size",
            SysctlKey::KernZleakZone_threshold => "kern.zleak.zone_threshold",
            SysctlKey::KperfDebug_level => "kperf.debug_level",
            SysctlKey::KperfLimitsTimer_min_bg_period_ns => "kperf.limits.timer_min_bg_period_ns",
            SysctlKey::KperfLimitsTimer_min_bg_pet_period_ns => "kperf.limits.timer_min_bg_pet_period_ns",
            SysctlKey::KperfLimitsTimer_min_period_ns => "kperf.limits.timer_min_period_ns",
            SysctlKey::KperfLimitsTimer_min_pet_period_ns => "kperf.limits.timer_min_pet_period_ns",
            SysctlKey::KtraceBackground_pid => "ktrace.background_pid",
            SysctlKey::KtraceConfigured_by => "ktrace.configured_by",
            SysctlKey::KtraceOwning_pid => "ktrace.owning_pid",
            SysctlKey::KtraceState => "ktrace.state",
            SysctlKey::MachdepCpuAddress_bitsPhysical => "machdep.cpu.address_bits.physical",
            SysctlKey::MachdepCpuAddress_bitsVirtual => "machdep.cpu.address_bits.virtual",
            SysctlKey::MachdepCpuArch_perfEvents => "machdep.cpu.arch_perf.events",
            SysctlKey::MachdepCpuArch_perfEvents_number => "machdep.cpu.arch_perf.events_number",
            SysctlKey::MachdepCpuArch_perfFixed_number => "machdep.cpu.arch_perf.fixed_number",
            SysctlKey::MachdepCpuArch_perfFixed_width => "machdep.cpu.arch_perf.fixed_width",
            SysctlKey::MachdepCpuArch_perfNumber => "machdep.cpu.arch_perf.number",
            SysctlKey::MachdepCpuArch_perfVersion => "machdep.cpu.arch_perf.version",
            SysctlKey::MachdepCpuArch_perfWidth => "machdep.cpu.arch_perf.width",
            SysctlKey::MachdepCpuBrand => "machdep.cpu.brand",
            SysctlKey::MachdepCpuBrand_string => "machdep.cpu.brand_string",
            SysctlKey::MachdepCpuCacheL2_associativity => "machdep.cpu.cache.L2_associativity",
            SysctlKey::MachdepCpuCacheLinesize => "machdep.cpu.cache.linesize",
            SysctlKey::MachdepCpuCacheSize => "machdep.cpu.cache.size",
            SysctlKey::MachdepCpuCore_count => "machdep.cpu.core_count",
            SysctlKey::MachdepCpuCores_per_package => "machdep.cpu.cores_per_package",
            SysctlKey::MachdepCpuExtfamily => "machdep.cpu.extfamily",
            SysctlKey::MachdepCpuExtfeature_bits => "machdep.cpu.extfeature_bits",
            SysctlKey::MachdepCpuExtfeatures => "machdep.cpu.extfeatures",
            SysctlKey::MachdepCpuExtmodel => "machdep.cpu.extmodel",
            SysctlKey::MachdepCpuFamily => "machdep.cpu.family",
            SysctlKey::MachdepCpuFeature_bits => "machdep.cpu.feature_bits",
            SysctlKey::MachdepCpuFeatures => "machdep.cpu.features",
            SysctlKey::MachdepCpuLeaf7_feature_bits => "machdep.cpu.leaf7_feature_bits",
            SysctlKey::MachdepCpuLeaf7_feature_bits_edx => "machdep.cpu.leaf7_feature_bits_edx",
            SysctlKey::MachdepCpuLeaf7_features => "machdep.cpu.leaf7_features",
            SysctlKey::MachdepCpuLogical_per_package => "machdep.cpu.logical_per_package",
            SysctlKey::MachdepCpuMax_basic => "machdep.cpu.max_basic",
            SysctlKey::MachdepCpuMax_ext => "machdep.cpu.max_ext",
            SysctlKey::MachdepCpuMicrocode_version => "machdep.cpu.microcode_version",
            SysctlKey::MachdepCpuModel => "machdep.cpu.model",
            SysctlKey::MachdepCpuMwaitExtensions => "machdep.cpu.mwait.extensions",
            SysctlKey::MachdepCpuMwaitLinesize_max => "machdep.cpu.mwait.linesize_max",
            SysctlKey::MachdepCpuMwaitLinesize_min => "machdep.cpu.mwait.linesize_min",
            SysctlKey::MachdepCpuMwaitSub_Cstates => "machdep.cpu.mwait.sub_Cstates",
            SysctlKey::MachdepCpuProcessor_flag => "machdep.cpu.processor_flag",
            SysctlKey::MachdepCpuSignature => "machdep.cpu.signature",
            SysctlKey::MachdepCpuStepping => "machdep.cpu.stepping",
            SysctlKey::MachdepCpuThermalACNT_MCNT => "machdep.cpu.thermal.ACNT_MCNT",
            SysctlKey::MachdepCpuThermalCore_power_limits => "machdep.cpu.thermal.core_power_limits",
            SysctlKey::MachdepCpuThermalDynamic_acceleration => "machdep.cpu.thermal.dynamic_acceleration",
            SysctlKey::MachdepCpuThermalEnergy_policy => "machdep.cpu.thermal.energy_policy",
            SysctlKey::MachdepCpuThermalFine_grain_clock_mod => "machdep.cpu.thermal.fine_grain_clock_mod",
            SysctlKey::MachdepCpuThermalHardware_feedback => "machdep.cpu.thermal.hardware_feedback",
            SysctlKey::MachdepCpuThermalInvariant_APIC_timer => "machdep.cpu.thermal.invariant_APIC_timer",
            SysctlKey::MachdepCpuThermalPackage_thermal_intr => "machdep.cpu.thermal.package_thermal_intr",
            SysctlKey::MachdepCpuThermalSensor => "machdep.cpu.thermal.sensor",
            SysctlKey::MachdepCpuThermalThresholds => "machdep.cpu.thermal.thresholds",
            SysctlKey::MachdepCpuThread_count => "machdep.cpu.thread_count",
            SysctlKey::MachdepCpuTlbDataSmall => "machdep.cpu.tlb.data.small",
            SysctlKey::MachdepCpuTlbDataSmall_level1 => "machdep.cpu.tlb.data.small_level1",
            SysctlKey::MachdepCpuTlbInstLarge => "machdep.cpu.tlb.inst.large",
            SysctlKey::MachdepCpuTsc_cccDenominator => "machdep.cpu.tsc_ccc.denominator",
            SysctlKey::MachdepCpuTsc_cccNumerator => "machdep.cpu.tsc_ccc.numerator",
            SysctlKey::MachdepCpuVendor => "machdep.cpu.vendor",
            SysctlKey::MachdepCpuXsaveExtended_state => "machdep.cpu.xsave.extended_state",
            SysctlKey::MachdepCpuXsaveExtended_state1 => "machdep.cpu.xsave.extended_state1",
            SysctlKey::MachdepEager_timer_evaluation_max => "machdep.eager_timer_evaluation_max",
            SysctlKey::MachdepEager_timer_evaluations => "machdep.eager_timer_evaluations",
            SysctlKey::MachdepMemmapACPINVS => "machdep.memmap.ACPINVS",
            SysctlKey::MachdepMemmapACPIReclaim => "machdep.memmap.ACPIReclaim",
            SysctlKey::MachdepMemmapConventional => "machdep.memmap.Conventional",
            SysctlKey::MachdepMemmapOther => "machdep.memmap.Other",
            SysctlKey::MachdepMemmapPalCode => "machdep.memmap.PalCode",
            SysctlKey::MachdepMemmapReserved => "machdep.memmap.Reserved",
            SysctlKey::MachdepMemmapRuntimeServices => "machdep.memmap.RuntimeServices",
            SysctlKey::MachdepMemmapUnusable => "machdep.memmap.Unusable",
            SysctlKey::MachdepMiscFast_uexc_support => "machdep.misc.fast_uexc_support",
            SysctlKey::MachdepMiscInterrupt_latency_max => "machdep.misc.interrupt_latency_max",
            SysctlKey::MachdepMiscNmis => "machdep.misc.nmis",
            SysctlKey::MachdepMiscPanic_restart_timeout => "machdep.misc.panic_restart_timeout",
            SysctlKey::MachdepMiscTimer_queue_trace => "machdep.misc.timer_queue_trace",
            SysctlKey::MachdepPmapHashcnts => "machdep.pmap.hashcnts",
            SysctlKey::MachdepPmapHashmax => "machdep.pmap.hashmax",
            SysctlKey::MachdepPmapHashwalks => "machdep.pmap.hashwalks",
            SysctlKey::MachdepPmapKern_pv_reserve => "machdep.pmap.kern_pv_reserve",
            SysctlKey::MachdepPmapKernel_text_ps => "machdep.pmap.kernel_text_ps",
            SysctlKey::MachdepTscAt_boot => "machdep.tsc.at_boot",
            SysctlKey::MachdepTscDeep_idle_rebase => "machdep.tsc.deep_idle_rebase",
            SysctlKey::MachdepTscFrequency => "machdep.tsc.frequency",
            SysctlKey::MachdepTscNanotimeGeneration => "machdep.tsc.nanotime.generation",
            SysctlKey::MachdepTscNanotimeNs_base => "machdep.tsc.nanotime.ns_base",
            SysctlKey::MachdepTscNanotimeScale => "machdep.tsc.nanotime.scale",
            SysctlKey::MachdepTscNanotimeShift => "machdep.tsc.nanotime.shift",
            SysctlKey::MachdepTscNanotimeTsc_base => "machdep.tsc.nanotime.tsc_base",
            SysctlKey::MachdepTscRebase_abs_time => "machdep.tsc.rebase_abs_time",
            SysctlKey::MachdepUser_idle_level => "machdep.user_idle_level",
            SysctlKey::MachdepVectorsIPI => "machdep.vectors.IPI",
            SysctlKey::MachdepVectorsTimer => "machdep.vectors.timer",
            SysctlKey::MachdepX86_fp_simd_isr_uses => "machdep.x86_fp_simd_isr_uses",
            SysctlKey::MachdepXcpmBootplim => "machdep.xcpm.bootplim",
            SysctlKey::MachdepXcpmBootpst => "machdep.xcpm.bootpst",
            SysctlKey::MachdepXcpmCpu_thermal_level => "machdep.xcpm.cpu_thermal_level",
            SysctlKey::MachdepXcpmDeep_idle_count => "machdep.xcpm.deep_idle_count",
            SysctlKey::MachdepXcpmDeep_idle_last_stats => "machdep.xcpm.deep_idle_last_stats",
            SysctlKey::MachdepXcpmDeep_idle_log => "machdep.xcpm.deep_idle_log",
            SysctlKey::MachdepXcpmDeep_idle_total_stats => "machdep.xcpm.deep_idle_total_stats",
            SysctlKey::MachdepXcpmEpp_override => "machdep.xcpm.epp_override",
            SysctlKey::MachdepXcpmForced_idle_period => "machdep.xcpm.forced_idle_period",
            SysctlKey::MachdepXcpmForced_idle_ratio => "machdep.xcpm.forced_idle_ratio",
            SysctlKey::MachdepXcpmGpu_thermal_level => "machdep.xcpm.gpu_thermal_level",
            SysctlKey::MachdepXcpmHard_plimit_max_100mhz_ratio => "machdep.xcpm.hard_plimit_max_100mhz_ratio",
            SysctlKey::MachdepXcpmHard_plimit_min_100mhz_ratio => "machdep.xcpm.hard_plimit_min_100mhz_ratio",
            SysctlKey::MachdepXcpmIo_control_disengages => "machdep.xcpm.io_control_disengages",
            SysctlKey::MachdepXcpmIo_control_engages => "machdep.xcpm.io_control_engages",
            SysctlKey::MachdepXcpmIo_cst_control_enabled => "machdep.xcpm.io_cst_control_enabled",
            SysctlKey::MachdepXcpmIo_epp_boost_enabled => "machdep.xcpm.io_epp_boost_enabled",
            SysctlKey::MachdepXcpmIo_filtered_reads => "machdep.xcpm.io_filtered_reads",
            SysctlKey::MachdepXcpmIo_thermal_level => "machdep.xcpm.io_thermal_level",
            SysctlKey::MachdepXcpmMaxbusdelay => "machdep.xcpm.maxbusdelay",
            SysctlKey::MachdepXcpmMaxintdelay => "machdep.xcpm.maxintdelay",
            SysctlKey::MachdepXcpmMbd_applications => "machdep.xcpm.mbd_applications",
            SysctlKey::MachdepXcpmMbd_mode => "machdep.xcpm.mbd_mode",
            SysctlKey::MachdepXcpmMbd_relaxations => "machdep.xcpm.mbd_relaxations",
            SysctlKey::MachdepXcpmMid_applications => "machdep.xcpm.mid_applications",
            SysctlKey::MachdepXcpmMid_cst_control_limit => "machdep.xcpm.mid_cst_control_limit",
            SysctlKey::MachdepXcpmMid_mode => "machdep.xcpm.mid_mode",
            SysctlKey::MachdepXcpmMid_mode_active => "machdep.xcpm.mid_mode_active",
            SysctlKey::MachdepXcpmMid_relaxations => "machdep.xcpm.mid_relaxations",
            SysctlKey::MachdepXcpmMode => "machdep.xcpm.mode",
            SysctlKey::MachdepXcpmPcps_mode => "machdep.xcpm.pcps_mode",
            SysctlKey::MachdepXcpmPcps_rt_override_mode => "machdep.xcpm.pcps_rt_override_mode",
            SysctlKey::MachdepXcpmPcps_rt_override_ns => "machdep.xcpm.pcps_rt_override_ns",
            SysctlKey::MachdepXcpmPerf_hints => "machdep.xcpm.perf_hints",
            SysctlKey::MachdepXcpmPower_source => "machdep.xcpm.power_source",
            SysctlKey::MachdepXcpmQos_txfr => "machdep.xcpm.qos_txfr",
            SysctlKey::MachdepXcpmRatio_change_ratelimit_ns => "machdep.xcpm.ratio_change_ratelimit_ns",
            SysctlKey::MachdepXcpmRatio_changes_total => "machdep.xcpm.ratio_changes_total",
            SysctlKey::MachdepXcpmRing_boost_enabled => "machdep.xcpm.ring_boost_enabled",
            SysctlKey::MachdepXcpmSoft_plimit_max_100mhz_ratio => "machdep.xcpm.soft_plimit_max_100mhz_ratio",
            SysctlKey::MachdepXcpmSoft_plimit_min_100mhz_ratio => "machdep.xcpm.soft_plimit_min_100mhz_ratio",
            SysctlKey::MachdepXcpmTuib_enabled => "machdep.xcpm.tuib_enabled",
            SysctlKey::MachdepXcpmTuib_ns => "machdep.xcpm.tuib_ns",
            SysctlKey::MachdepXcpmTuib_plimit_max_100mhz_ratio => "machdep.xcpm.tuib_plimit_max_100mhz_ratio",
            SysctlKey::MachdepXcpmTuib_plimit_min_100mhz_ratio => "machdep.xcpm.tuib_plimit_min_100mhz_ratio",
            SysctlKey::MachdepXcpmVectors_loaded_count => "machdep.xcpm.vectors_loaded_count",
            SysctlKey::NetAlfDefaultaction => "net.alf.defaultaction",
            SysctlKey::NetAlfLoglevel => "net.alf.loglevel",
            SysctlKey::NetAlfMqcount => "net.alf.mqcount",
            SysctlKey::NetAlfPerm => "net.alf.perm",
            SysctlKey::NetCfilActive_count => "net.cfil.active_count",
            SysctlKey::NetCfilClose_wait_timeout => "net.cfil.close_wait_timeout",
            SysctlKey::NetCfilDebug => "net.cfil.debug",
            SysctlKey::NetCfilLog => "net.cfil.log",
            SysctlKey::NetCfilSbtrim => "net.cfil.sbtrim",
            SysctlKey::NetCfilSock_attached_count => "net.cfil.sock_attached_count",
            SysctlKey::NetClassqSfbAllocation => "net.classq.sfb.allocation",
            SysctlKey::NetClassqSfbDecrement => "net.classq.sfb.decrement",
            SysctlKey::NetClassqSfbHinterval => "net.classq.sfb.hinterval",
            SysctlKey::NetClassqSfbHoldtime => "net.classq.sfb.holdtime",
            SysctlKey::NetClassqSfbIncrement => "net.classq.sfb.increment",
            SysctlKey::NetClassqSfbPboxtime => "net.classq.sfb.pboxtime",
            SysctlKey::NetClassqSfbRatelimit => "net.classq.sfb.ratelimit",
            SysctlKey::NetClassqTarget_qdelay => "net.classq.target_qdelay",
            SysctlKey::NetClassqUpdate_interval => "net.classq.update_interval",
            SysctlKey::NetClassqVerbose => "net.classq.verbose",
            SysctlKey::NetInet6Icmp6Errppslimit => "net.inet6.icmp6.errppslimit",
            SysctlKey::NetInet6Icmp6Nd6_accept_6to4 => "net.inet6.icmp6.nd6_accept_6to4",
            SysctlKey::NetInet6Icmp6Nd6_debug => "net.inet6.icmp6.nd6_debug",
            SysctlKey::NetInet6Icmp6Nd6_delay => "net.inet6.icmp6.nd6_delay",
            SysctlKey::NetInet6Icmp6Nd6_llreach_base => "net.inet6.icmp6.nd6_llreach_base",
            SysctlKey::NetInet6Icmp6Nd6_maxproxiedsol => "net.inet6.icmp6.nd6_maxproxiedsol",
            SysctlKey::NetInet6Icmp6Nd6_maxsolstgt => "net.inet6.icmp6.nd6_maxsolstgt",
            SysctlKey::NetInet6Icmp6Nd6_mmaxtries => "net.inet6.icmp6.nd6_mmaxtries",
            SysctlKey::NetInet6Icmp6Nd6_onlink_ns_rfc4861 => "net.inet6.icmp6.nd6_onlink_ns_rfc4861",
            SysctlKey::NetInet6Icmp6Nd6_optimistic_dad => "net.inet6.icmp6.nd6_optimistic_dad",
            SysctlKey::NetInet6Icmp6Nd6_prune => "net.inet6.icmp6.nd6_prune",
            SysctlKey::NetInet6Icmp6Nd6_prune_lazy => "net.inet6.icmp6.nd6_prune_lazy",
            SysctlKey::NetInet6Icmp6Nd6_umaxtries => "net.inet6.icmp6.nd6_umaxtries",
            SysctlKey::NetInet6Icmp6Nd6_useloopback => "net.inet6.icmp6.nd6_useloopback",
            SysctlKey::NetInet6Icmp6Nodeinfo => "net.inet6.icmp6.nodeinfo",
            SysctlKey::NetInet6Icmp6Prproxy_cnt => "net.inet6.icmp6.prproxy_cnt",
            SysctlKey::NetInet6Icmp6Rappslimit => "net.inet6.icmp6.rappslimit",
            SysctlKey::NetInet6Icmp6Rediraccept => "net.inet6.icmp6.rediraccept",
            SysctlKey::NetInet6Icmp6Redirtimeout => "net.inet6.icmp6.redirtimeout",
            SysctlKey::NetInet6Ip6Accept_rtadv => "net.inet6.ip6.accept_rtadv",
            SysctlKey::NetInet6Ip6Adj_clear_hwcksum => "net.inet6.ip6.adj_clear_hwcksum",
            SysctlKey::NetInet6Ip6Adj_partial_sum => "net.inet6.ip6.adj_partial_sum",
            SysctlKey::NetInet6Ip6Auto_flowlabel => "net.inet6.ip6.auto_flowlabel",
            SysctlKey::NetInet6Ip6Auto_linklocal => "net.inet6.ip6.auto_linklocal",
            SysctlKey::NetInet6Ip6Check_interface => "net.inet6.ip6.check_interface",
            SysctlKey::NetInet6Ip6Checkinterface_debug => "net.inet6.ip6.checkinterface_debug",
            SysctlKey::NetInet6Ip6Clat_debug => "net.inet6.ip6.clat_debug",
            SysctlKey::NetInet6Ip6Dad_count => "net.inet6.ip6.dad_count",
            SysctlKey::NetInet6Ip6Dad_enhanced => "net.inet6.ip6.dad_enhanced",
            SysctlKey::NetInet6Ip6Defmcasthlim => "net.inet6.ip6.defmcasthlim",
            SysctlKey::NetInet6Ip6Forwarding => "net.inet6.ip6.forwarding",
            SysctlKey::NetInet6Ip6Fragpackets => "net.inet6.ip6.fragpackets",
            SysctlKey::NetInet6Ip6Gifhlim => "net.inet6.ip6.gifhlim",
            SysctlKey::NetInet6Ip6Hdrnestlimit => "net.inet6.ip6.hdrnestlimit",
            SysctlKey::NetInet6Ip6Hlim => "net.inet6.ip6.hlim",
            SysctlKey::NetInet6Ip6Input_perf => "net.inet6.ip6.input_perf",
            SysctlKey::NetInet6Ip6Input_perf_bins => "net.inet6.ip6.input_perf_bins",
            SysctlKey::NetInet6Ip6Kame_version => "net.inet6.ip6.kame_version",
            SysctlKey::NetInet6Ip6Keepfaith => "net.inet6.ip6.keepfaith",
            SysctlKey::NetInet6Ip6Log_interval => "net.inet6.ip6.log_interval",
            SysctlKey::NetInet6Ip6Maxchainsent => "net.inet6.ip6.maxchainsent",
            SysctlKey::NetInet6Ip6Maxdynroutes => "net.inet6.ip6.maxdynroutes",
            SysctlKey::NetInet6Ip6Maxfragpackets => "net.inet6.ip6.maxfragpackets",
            SysctlKey::NetInet6Ip6Maxfrags => "net.inet6.ip6.maxfrags",
            SysctlKey::NetInet6Ip6Maxifdefrouters => "net.inet6.ip6.maxifdefrouters",
            SysctlKey::NetInet6Ip6Maxifprefixes => "net.inet6.ip6.maxifprefixes",
            SysctlKey::NetInet6Ip6McastLoop => "net.inet6.ip6.mcast.loop",
            SysctlKey::NetInet6Ip6McastMaxgrpsrc => "net.inet6.ip6.mcast.maxgrpsrc",
            SysctlKey::NetInet6Ip6McastMaxsocksrc => "net.inet6.ip6.mcast.maxsocksrc",
            SysctlKey::NetInet6Ip6Mcast_pmtu => "net.inet6.ip6.mcast_pmtu",
            SysctlKey::NetInet6Ip6Neighborgcthresh => "net.inet6.ip6.neighborgcthresh",
            SysctlKey::NetInet6Ip6Only_allow_rfc4193_prefixes => "net.inet6.ip6.only_allow_rfc4193_prefixes",
            SysctlKey::NetInet6Ip6Output_perf => "net.inet6.ip6.output_perf",
            SysctlKey::NetInet6Ip6Output_perf_bins => "net.inet6.ip6.output_perf_bins",
            SysctlKey::NetInet6Ip6Prefer_tempaddr => "net.inet6.ip6.prefer_tempaddr",
            SysctlKey::NetInet6Ip6Redirect => "net.inet6.ip6.redirect",
            SysctlKey::NetInet6Ip6Rr_prune => "net.inet6.ip6.rr_prune",
            SysctlKey::NetInet6Ip6Rtexpire => "net.inet6.ip6.rtexpire",
            SysctlKey::NetInet6Ip6Rtmaxcache => "net.inet6.ip6.rtmaxcache",
            SysctlKey::NetInet6Ip6Rtminexpire => "net.inet6.ip6.rtminexpire",
            SysctlKey::NetInet6Ip6Select_src_expensive_secondary_if => "net.inet6.ip6.select_src_expensive_secondary_if",
            SysctlKey::NetInet6Ip6Select_src_strong_end => "net.inet6.ip6.select_src_strong_end",
            SysctlKey::NetInet6Ip6Select_srcaddr_debug => "net.inet6.ip6.select_srcaddr_debug",
            SysctlKey::NetInet6Ip6Select_srcif_debug => "net.inet6.ip6.select_srcif_debug",
            SysctlKey::NetInet6Ip6Temppltime => "net.inet6.ip6.temppltime",
            SysctlKey::NetInet6Ip6Tempvltime => "net.inet6.ip6.tempvltime",
            SysctlKey::NetInet6Ip6Use_defaultzone => "net.inet6.ip6.use_defaultzone",
            SysctlKey::NetInet6Ip6Use_deprecated => "net.inet6.ip6.use_deprecated",
            SysctlKey::NetInet6Ip6Use_tempaddr => "net.inet6.ip6.use_tempaddr",
            SysctlKey::NetInet6Ip6V6only => "net.inet6.ip6.v6only",
            SysctlKey::NetInet6Ipsec6Ah_net_deflev => "net.inet6.ipsec6.ah_net_deflev",
            SysctlKey::NetInet6Ipsec6Ah_trans_deflev => "net.inet6.ipsec6.ah_trans_deflev",
            SysctlKey::NetInet6Ipsec6Debug => "net.inet6.ipsec6.debug",
            SysctlKey::NetInet6Ipsec6Def_policy => "net.inet6.ipsec6.def_policy",
            SysctlKey::NetInet6Ipsec6Ecn => "net.inet6.ipsec6.ecn",
            SysctlKey::NetInet6Ipsec6Esp_net_deflev => "net.inet6.ipsec6.esp_net_deflev",
            SysctlKey::NetInet6Ipsec6Esp_randpad => "net.inet6.ipsec6.esp_randpad",
            SysctlKey::NetInet6Ipsec6Esp_trans_deflev => "net.inet6.ipsec6.esp_trans_deflev",
            SysctlKey::NetInet6MldDebug => "net.inet6.mld.debug",
            SysctlKey::NetInet6MldGsrdelay => "net.inet6.mld.gsrdelay",
            SysctlKey::NetInet6MldUse_allow => "net.inet6.mld.use_allow",
            SysctlKey::NetInet6MldV1enable => "net.inet6.mld.v1enable",
            SysctlKey::NetInet6MldV2enable => "net.inet6.mld.v2enable",
            SysctlKey::NetInet6SendOpmode => "net.inet6.send.opmode",
            SysctlKey::NetInet6SendOpstate => "net.inet6.send.opstate",
            SysctlKey::NetInetIcmpBmcastecho => "net.inet.icmp.bmcastecho",
            SysctlKey::NetInetIcmpDrop_redirect => "net.inet.icmp.drop_redirect",
            SysctlKey::NetInetIcmpIcmplim => "net.inet.icmp.icmplim",
            SysctlKey::NetInetIcmpLog_redirect => "net.inet.icmp.log_redirect",
            SysctlKey::NetInetIcmpMaskrepl => "net.inet.icmp.maskrepl",
            SysctlKey::NetInetIcmpTimestamp => "net.inet.icmp.timestamp",
            SysctlKey::NetInetIgmpDebug => "net.inet.igmp.debug",
            SysctlKey::NetInetIgmpDefault_version => "net.inet.igmp.default_version",
            SysctlKey::NetInetIgmpGsrdelay => "net.inet.igmp.gsrdelay",
            SysctlKey::NetInetIgmpLegacysupp => "net.inet.igmp.legacysupp",
            SysctlKey::NetInetIgmpRecvifkludge => "net.inet.igmp.recvifkludge",
            SysctlKey::NetInetIgmpSendlocal => "net.inet.igmp.sendlocal",
            SysctlKey::NetInetIgmpSendra => "net.inet.igmp.sendra",
            SysctlKey::NetInetIgmpV1enable => "net.inet.igmp.v1enable",
            SysctlKey::NetInetIgmpV2enable => "net.inet.igmp.v2enable",
            SysctlKey::NetInetIpAccept_sourceroute => "net.inet.ip.accept_sourceroute",
            SysctlKey::NetInetIpAdj_clear_hwcksum => "net.inet.ip.adj_clear_hwcksum",
            SysctlKey::NetInetIpAdj_partial_sum => "net.inet.ip.adj_partial_sum",
            SysctlKey::NetInetIpCheck_interface => "net.inet.ip.check_interface",
            SysctlKey::NetInetIpCheckinterface_debug => "net.inet.ip.checkinterface_debug",
            SysctlKey::NetInetIpDummynetCurr_time => "net.inet.ip.dummynet.curr_time",
            SysctlKey::NetInetIpDummynetDebug => "net.inet.ip.dummynet.debug",
            SysctlKey::NetInetIpDummynetExpire => "net.inet.ip.dummynet.expire",
            SysctlKey::NetInetIpDummynetExtract_heap => "net.inet.ip.dummynet.extract_heap",
            SysctlKey::NetInetIpDummynetHash_size => "net.inet.ip.dummynet.hash_size",
            SysctlKey::NetInetIpDummynetMax_chain_len => "net.inet.ip.dummynet.max_chain_len",
            SysctlKey::NetInetIpDummynetReady_heap => "net.inet.ip.dummynet.ready_heap",
            SysctlKey::NetInetIpDummynetRed_avg_pkt_size => "net.inet.ip.dummynet.red_avg_pkt_size",
            SysctlKey::NetInetIpDummynetRed_lookup_depth => "net.inet.ip.dummynet.red_lookup_depth",
            SysctlKey::NetInetIpDummynetRed_max_pkt_size => "net.inet.ip.dummynet.red_max_pkt_size",
            SysctlKey::NetInetIpDummynetSearch_steps => "net.inet.ip.dummynet.search_steps",
            SysctlKey::NetInetIpDummynetSearches => "net.inet.ip.dummynet.searches",
            SysctlKey::NetInetIpForwarding => "net.inet.ip.forwarding",
            SysctlKey::NetInetIpFragpackets => "net.inet.ip.fragpackets",
            SysctlKey::NetInetIpGifttl => "net.inet.ip.gifttl",
            SysctlKey::NetInetIpLinklocalInAllowbadttl => "net.inet.ip.linklocal.in.allowbadttl",
            SysctlKey::NetInetIpMaxchainsent => "net.inet.ip.maxchainsent",
            SysctlKey::NetInetIpMaxfragpackets => "net.inet.ip.maxfragpackets",
            SysctlKey::NetInetIpMaxfragsperpacket => "net.inet.ip.maxfragsperpacket",
            SysctlKey::NetInetIpMcastLoop => "net.inet.ip.mcast.loop",
            SysctlKey::NetInetIpMcastMaxgrpsrc => "net.inet.ip.mcast.maxgrpsrc",
            SysctlKey::NetInetIpMcastMaxsocksrc => "net.inet.ip.mcast.maxsocksrc",
            SysctlKey::NetInetIpOutput_perf => "net.inet.ip.output_perf",
            SysctlKey::NetInetIpOutput_perf_bins => "net.inet.ip.output_perf_bins",
            SysctlKey::NetInetIpPortrangeFirst => "net.inet.ip.portrange.first",
            SysctlKey::NetInetIpPortrangeHifirst => "net.inet.ip.portrange.hifirst",
            SysctlKey::NetInetIpPortrangeHilast => "net.inet.ip.portrange.hilast",
            SysctlKey::NetInetIpPortrangeLast => "net.inet.ip.portrange.last",
            SysctlKey::NetInetIpPortrangeLowfirst => "net.inet.ip.portrange.lowfirst",
            SysctlKey::NetInetIpPortrangeLowlast => "net.inet.ip.portrange.lowlast",
            SysctlKey::NetInetIpRandom_id => "net.inet.ip.random_id",
            SysctlKey::NetInetIpRandom_id_collisions => "net.inet.ip.random_id_collisions",
            SysctlKey::NetInetIpRandom_id_statistics => "net.inet.ip.random_id_statistics",
            SysctlKey::NetInetIpRandom_id_total => "net.inet.ip.random_id_total",
            SysctlKey::NetInetIpRedirect => "net.inet.ip.redirect",
            SysctlKey::NetInetIpRfc6864 => "net.inet.ip.rfc6864",
            SysctlKey::NetInetIpRtexpire => "net.inet.ip.rtexpire",
            SysctlKey::NetInetIpRtmaxcache => "net.inet.ip.rtmaxcache",
            SysctlKey::NetInetIpRtminexpire => "net.inet.ip.rtminexpire",
            SysctlKey::NetInetIpRx_chaining => "net.inet.ip.rx_chaining",
            SysctlKey::NetInetIpRx_chainsz => "net.inet.ip.rx_chainsz",
            SysctlKey::NetInetIpSelect_srcif_debug => "net.inet.ip.select_srcif_debug",
            SysctlKey::NetInetIpSendsourcequench => "net.inet.ip.sendsourcequench",
            SysctlKey::NetInetIpSourceroute => "net.inet.ip.sourceroute",
            SysctlKey::NetInetIpSubnets_are_local => "net.inet.ip.subnets_are_local",
            SysctlKey::NetInetIpTtl => "net.inet.ip.ttl",
            SysctlKey::NetInetIpsecAh_cleartos => "net.inet.ipsec.ah_cleartos",
            SysctlKey::NetInetIpsecAh_net_deflev => "net.inet.ipsec.ah_net_deflev",
            SysctlKey::NetInetIpsecAh_offsetmask => "net.inet.ipsec.ah_offsetmask",
            SysctlKey::NetInetIpsecAh_trans_deflev => "net.inet.ipsec.ah_trans_deflev",
            SysctlKey::NetInetIpsecBypass => "net.inet.ipsec.bypass",
            SysctlKey::NetInetIpsecDebug => "net.inet.ipsec.debug",
            SysctlKey::NetInetIpsecDef_policy => "net.inet.ipsec.def_policy",
            SysctlKey::NetInetIpsecDfbit => "net.inet.ipsec.dfbit",
            SysctlKey::NetInetIpsecEcn => "net.inet.ipsec.ecn",
            SysctlKey::NetInetIpsecEsp_net_deflev => "net.inet.ipsec.esp_net_deflev",
            SysctlKey::NetInetIpsecEsp_port => "net.inet.ipsec.esp_port",
            SysctlKey::NetInetIpsecEsp_randpad => "net.inet.ipsec.esp_randpad",
            SysctlKey::NetInetIpsecEsp_trans_deflev => "net.inet.ipsec.esp_trans_deflev",
            SysctlKey::NetInetLog_restricted => "net.inet.log_restricted",
            SysctlKey::NetInetMptcpAllow_aggregate => "net.inet.mptcp.allow_aggregate",
            SysctlKey::NetInetMptcpAlternate_port => "net.inet.mptcp.alternate_port",
            SysctlKey::NetInetMptcpDbg_area => "net.inet.mptcp.dbg_area",
            SysctlKey::NetInetMptcpDbg_level => "net.inet.mptcp.dbg_level",
            SysctlKey::NetInetMptcpDss_csum => "net.inet.mptcp.dss_csum",
            SysctlKey::NetInetMptcpEnable => "net.inet.mptcp.enable",
            SysctlKey::NetInetMptcpExpected_progress_headstart => "net.inet.mptcp.expected_progress_headstart",
            SysctlKey::NetInetMptcpFail => "net.inet.mptcp.fail",
            SysctlKey::NetInetMptcpKeepalive => "net.inet.mptcp.keepalive",
            SysctlKey::NetInetMptcpMptcp_cap_retr => "net.inet.mptcp.mptcp_cap_retr",
            SysctlKey::NetInetMptcpNrto => "net.inet.mptcp.nrto",
            SysctlKey::NetInetMptcpPcbcount => "net.inet.mptcp.pcbcount",
            SysctlKey::NetInetMptcpProbecnt => "net.inet.mptcp.probecnt",
            SysctlKey::NetInetMptcpProbeto => "net.inet.mptcp.probeto",
            SysctlKey::NetInetMptcpRto => "net.inet.mptcp.rto",
            SysctlKey::NetInetMptcpRto_thresh => "net.inet.mptcp.rto_thresh",
            SysctlKey::NetInetMptcpRtthist_thresh => "net.inet.mptcp.rtthist_thresh",
            SysctlKey::NetInetMptcpTw => "net.inet.mptcp.tw",
            SysctlKey::NetInetMptcpUserto => "net.inet.mptcp.userto",
            SysctlKey::NetInetRawMaxdgram => "net.inet.raw.maxdgram",
            SysctlKey::NetInetRawPcbcount => "net.inet.raw.pcbcount",
            SysctlKey::NetInetRawRecvspace => "net.inet.raw.recvspace",
            SysctlKey::NetInetTcpAcc_iaj_react_limit => "net.inet.tcp.acc_iaj_react_limit",
            SysctlKey::NetInetTcpAck_prioritize => "net.inet.tcp.ack_prioritize",
            SysctlKey::NetInetTcpAlways_keepalive => "net.inet.tcp.always_keepalive",
            SysctlKey::NetInetTcpAutorcvbufmax => "net.inet.tcp.autorcvbufmax",
            SysctlKey::NetInetTcpAutosndbufinc => "net.inet.tcp.autosndbufinc",
            SysctlKey::NetInetTcpAutosndbufmax => "net.inet.tcp.autosndbufmax",
            SysctlKey::NetInetTcpAutotunereorder => "net.inet.tcp.autotunereorder",
            SysctlKey::NetInetTcpBackground_sockets => "net.inet.tcp.background_sockets",
            SysctlKey::NetInetTcpBackoff_maximum => "net.inet.tcp.backoff_maximum",
            SysctlKey::NetInetTcpBg_allowed_increase => "net.inet.tcp.bg_allowed_increase",
            SysctlKey::NetInetTcpBg_ss_fltsz => "net.inet.tcp.bg_ss_fltsz",
            SysctlKey::NetInetTcpBg_target_qdelay => "net.inet.tcp.bg_target_qdelay",
            SysctlKey::NetInetTcpBg_tether_shift => "net.inet.tcp.bg_tether_shift",
            SysctlKey::NetInetTcpBlackhole => "net.inet.tcp.blackhole",
            SysctlKey::NetInetTcpBroken_peer_syn_rexmit_thres => "net.inet.tcp.broken_peer_syn_rexmit_thres",
            SysctlKey::NetInetTcpCc_debug => "net.inet.tcp.cc_debug",
            SysctlKey::NetInetTcpChallengeack_limit => "net.inet.tcp.challengeack_limit",
            SysctlKey::NetInetTcpClear_tfocache => "net.inet.tcp.clear_tfocache",
            SysctlKey::NetInetTcpCubic_fast_convergence => "net.inet.tcp.cubic_fast_convergence",
            SysctlKey::NetInetTcpCubic_sockets => "net.inet.tcp.cubic_sockets",
            SysctlKey::NetInetTcpCubic_tcp_friendliness => "net.inet.tcp.cubic_tcp_friendliness",
            SysctlKey::NetInetTcpCubic_use_minrtt => "net.inet.tcp.cubic_use_minrtt",
            SysctlKey::NetInetTcpDelayed_ack => "net.inet.tcp.delayed_ack",
            SysctlKey::NetInetTcpDisable_access_to_stats => "net.inet.tcp.disable_access_to_stats",
            SysctlKey::NetInetTcpDisable_tcp_heuristics => "net.inet.tcp.disable_tcp_heuristics",
            SysctlKey::NetInetTcpDo_rfc5961 => "net.inet.tcp.do_rfc5961",
            SysctlKey::NetInetTcpDo_tcpdrain => "net.inet.tcp.do_tcpdrain",
            SysctlKey::NetInetTcpDoautorcvbuf => "net.inet.tcp.doautorcvbuf",
            SysctlKey::NetInetTcpDoautosndbuf => "net.inet.tcp.doautosndbuf",
            SysctlKey::NetInetTcpDrop_synfin => "net.inet.tcp.drop_synfin",
            SysctlKey::NetInetTcpEcn_initiate_out => "net.inet.tcp.ecn_initiate_out",
            SysctlKey::NetInetTcpEcn_negotiate_in => "net.inet.tcp.ecn_negotiate_in",
            SysctlKey::NetInetTcpEcn_setup_percentage => "net.inet.tcp.ecn_setup_percentage",
            SysctlKey::NetInetTcpEcn_timeout => "net.inet.tcp.ecn_timeout",
            SysctlKey::NetInetTcpEnable_tlp => "net.inet.tcp.enable_tlp",
            SysctlKey::NetInetTcpFastopen => "net.inet.tcp.fastopen",
            SysctlKey::NetInetTcpFastopen_backlog => "net.inet.tcp.fastopen_backlog",
            SysctlKey::NetInetTcpIcmp_may_rst => "net.inet.tcp.icmp_may_rst",
            SysctlKey::NetInetTcpInit_rtt_from_cache => "net.inet.tcp.init_rtt_from_cache",
            SysctlKey::NetInetTcpKeepcnt => "net.inet.tcp.keepcnt",
            SysctlKey::NetInetTcpKeepidle => "net.inet.tcp.keepidle",
            SysctlKey::NetInetTcpKeepinit => "net.inet.tcp.keepinit",
            SysctlKey::NetInetTcpKeepintvl => "net.inet.tcp.keepintvl",
            SysctlKey::NetInetTcpLocal_slowstart_flightsize => "net.inet.tcp.local_slowstart_flightsize",
            SysctlKey::NetInetTcpLogEnable => "net.inet.tcp.log.enable",
            SysctlKey::NetInetTcpLogEnable_usage => "net.inet.tcp.log.enable_usage",
            SysctlKey::NetInetTcpLogLevel_info => "net.inet.tcp.log.level_info",
            SysctlKey::NetInetTcpLogPrivacy => "net.inet.tcp.log.privacy",
            SysctlKey::NetInetTcpLogRate_current => "net.inet.tcp.log.rate_current",
            SysctlKey::NetInetTcpLogRate_duration => "net.inet.tcp.log.rate_duration",
            SysctlKey::NetInetTcpLogRate_exceeded_total => "net.inet.tcp.log.rate_exceeded_total",
            SysctlKey::NetInetTcpLogRate_limit => "net.inet.tcp.log.rate_limit",
            SysctlKey::NetInetTcpLogRate_max => "net.inet.tcp.log.rate_max",
            SysctlKey::NetInetTcpLogRtt_port => "net.inet.tcp.log.rtt_port",
            SysctlKey::NetInetTcpLogThflags_if_family => "net.inet.tcp.log.thflags_if_family",
            SysctlKey::NetInetTcpLog_in_vain => "net.inet.tcp.log_in_vain",
            SysctlKey::NetInetTcpLro => "net.inet.tcp.lro",
            SysctlKey::NetInetTcpLro_startcnt => "net.inet.tcp.lro_startcnt",
            SysctlKey::NetInetTcpLro_sz => "net.inet.tcp.lro_sz",
            SysctlKey::NetInetTcpLro_time => "net.inet.tcp.lro_time",
            SysctlKey::NetInetTcpLrodbg => "net.inet.tcp.lrodbg",
            SysctlKey::NetInetTcpMax_persist_timeout => "net.inet.tcp.max_persist_timeout",
            SysctlKey::NetInetTcpMaxseg_unacked => "net.inet.tcp.maxseg_unacked",
            SysctlKey::NetInetTcpMicrouptime_init => "net.inet.tcp.microuptime_init",
            SysctlKey::NetInetTcpMin_iaj_win => "net.inet.tcp.min_iaj_win",
            SysctlKey::NetInetTcpMinmss => "net.inet.tcp.minmss",
            SysctlKey::NetInetTcpMsl => "net.inet.tcp.msl",
            SysctlKey::NetInetTcpMssdflt => "net.inet.tcp.mssdflt",
            SysctlKey::NetInetTcpNewreno_sockets => "net.inet.tcp.newreno_sockets",
            SysctlKey::NetInetTcpNow_init => "net.inet.tcp.now_init",
            SysctlKey::NetInetTcpPacketchain => "net.inet.tcp.packetchain",
            SysctlKey::NetInetTcpPath_mtu_discovery => "net.inet.tcp.path_mtu_discovery",
            SysctlKey::NetInetTcpPcbcount => "net.inet.tcp.pcbcount",
            SysctlKey::NetInetTcpPmtud_blackhole_detection => "net.inet.tcp.pmtud_blackhole_detection",
            SysctlKey::NetInetTcpPmtud_blackhole_mss => "net.inet.tcp.pmtud_blackhole_mss",
            SysctlKey::NetInetTcpRandomize_ports => "net.inet.tcp.randomize_ports",
            SysctlKey::NetInetTcpRcvsspktcnt => "net.inet.tcp.rcvsspktcnt",
            SysctlKey::NetInetTcpReassOverflows => "net.inet.tcp.reass.overflows",
            SysctlKey::NetInetTcpRecv_allowed_iaj => "net.inet.tcp.recv_allowed_iaj",
            SysctlKey::NetInetTcpRecv_throttle_minwin => "net.inet.tcp.recv_throttle_minwin",
            SysctlKey::NetInetTcpRecvbg => "net.inet.tcp.recvbg",
            SysctlKey::NetInetTcpRecvspace => "net.inet.tcp.recvspace",
            SysctlKey::NetInetTcpRexmt_slop => "net.inet.tcp.rexmt_slop",
            SysctlKey::NetInetTcpRexmt_thresh => "net.inet.tcp.rexmt_thresh",
            SysctlKey::NetInetTcpRfc1644 => "net.inet.tcp.rfc1644",
            SysctlKey::NetInetTcpRfc3390 => "net.inet.tcp.rfc3390",
            SysctlKey::NetInetTcpRfc3465 => "net.inet.tcp.rfc3465",
            SysctlKey::NetInetTcpRfc3465_lim2 => "net.inet.tcp.rfc3465_lim2",
            SysctlKey::NetInetTcpRtt_min => "net.inet.tcp.rtt_min",
            SysctlKey::NetInetTcpRtt_recvbg => "net.inet.tcp.rtt_recvbg",
            SysctlKey::NetInetTcpSack => "net.inet.tcp.sack",
            SysctlKey::NetInetTcpSack_globalholes => "net.inet.tcp.sack_globalholes",
            SysctlKey::NetInetTcpSack_globalmaxholes => "net.inet.tcp.sack_globalmaxholes",
            SysctlKey::NetInetTcpSack_maxholes => "net.inet.tcp.sack_maxholes",
            SysctlKey::NetInetTcpSendspace => "net.inet.tcp.sendspace",
            SysctlKey::NetInetTcpSlowlink_wsize => "net.inet.tcp.slowlink_wsize",
            SysctlKey::NetInetTcpSlowstart_flightsize => "net.inet.tcp.slowstart_flightsize",
            SysctlKey::NetInetTcpSocket_unlocked_on_output => "net.inet.tcp.socket_unlocked_on_output",
            SysctlKey::NetInetTcpTcbhashsize => "net.inet.tcp.tcbhashsize",
            SysctlKey::NetInetTcpTcp_lq_overflow => "net.inet.tcp.tcp_lq_overflow",
            SysctlKey::NetInetTcpTcp_resched_timerlist => "net.inet.tcp.tcp_resched_timerlist",
            SysctlKey::NetInetTcpTcp_timer_advanced => "net.inet.tcp.tcp_timer_advanced",
            SysctlKey::NetInetTcpTimer_fastmode_idlemax => "net.inet.tcp.timer_fastmode_idlemax",
            SysctlKey::NetInetTcpTso => "net.inet.tcp.tso",
            SysctlKey::NetInetTcpTw_pcbcount => "net.inet.tcp.tw_pcbcount",
            SysctlKey::NetInetTcpUse_newreno => "net.inet.tcp.use_newreno",
            SysctlKey::NetInetTcpV6mssdflt => "net.inet.tcp.v6mssdflt",
            SysctlKey::NetInetTcpWin_scale_factor => "net.inet.tcp.win_scale_factor",
            SysctlKey::NetInetUdpBlackhole => "net.inet.udp.blackhole",
            SysctlKey::NetInetUdpChecksum => "net.inet.udp.checksum",
            SysctlKey::NetInetUdpLog_in_vain => "net.inet.udp.log_in_vain",
            SysctlKey::NetInetUdpMaxdgram => "net.inet.udp.maxdgram",
            SysctlKey::NetInetUdpPcbcount => "net.inet.udp.pcbcount",
            SysctlKey::NetInetUdpRandomize_ports => "net.inet.udp.randomize_ports",
            SysctlKey::NetInetUdpRecvspace => "net.inet.udp.recvspace",
            SysctlKey::NetIpsecDebug => "net.ipsec.debug",
            SysctlKey::NetIpsecMax_pending_input => "net.ipsec.max_pending_input",
            SysctlKey::NetIpsecRing_size => "net.ipsec.ring_size",
            SysctlKey::NetIpsecRx_fsw_ring_size => "net.ipsec.rx_fsw_ring_size",
            SysctlKey::NetIpsecTx_fsw_ring_size => "net.ipsec.tx_fsw_ring_size",
            SysctlKey::NetIpsecVerify_interface_creation => "net.ipsec.verify_interface_creation",
            SysctlKey::NetKeyAh_keymin => "net.key.ah_keymin",
            SysctlKey::NetKeyBlockacq_count => "net.key.blockacq_count",
            SysctlKey::NetKeyBlockacq_lifetime => "net.key.blockacq_lifetime",
            SysctlKey::NetKeyDebug => "net.key.debug",
            SysctlKey::NetKeyEsp_auth => "net.key.esp_auth",
            SysctlKey::NetKeyEsp_keymin => "net.key.esp_keymin",
            SysctlKey::NetKeyInt_random => "net.key.int_random",
            SysctlKey::NetKeyLarval_lifetime => "net.key.larval_lifetime",
            SysctlKey::NetKeyNatt_keepalive_interval => "net.key.natt_keepalive_interval",
            SysctlKey::NetKeyPrefered_oldsa => "net.key.prefered_oldsa",
            SysctlKey::NetKeySpi_maxval => "net.key.spi_maxval",
            SysctlKey::NetKeySpi_minval => "net.key.spi_minval",
            SysctlKey::NetKeySpi_trycnt => "net.key.spi_trycnt",
            SysctlKey::NetLinkBondDebug => "net.link.bond.debug",
            SysctlKey::NetLinkBridgeDebug => "net.link.bridge.debug",
            SysctlKey::NetLinkBridgeInherit_mac => "net.link.bridge.inherit_mac",
            SysctlKey::NetLinkBridgeRtable_hash_size_max => "net.link.bridge.rtable_hash_size_max",
            SysctlKey::NetLinkBridgeRtable_prune_period => "net.link.bridge.rtable_prune_period",
            SysctlKey::NetLinkBridgeTxstart => "net.link.bridge.txstart",
            SysctlKey::NetLinkEtherInetArp_llreach_base => "net.link.ether.inet.arp_llreach_base",
            SysctlKey::NetLinkEtherInetArp_unicast_lim => "net.link.ether.inet.arp_unicast_lim",
            SysctlKey::NetLinkEtherInetHost_down_time => "net.link.ether.inet.host_down_time",
            SysctlKey::NetLinkEtherInetKeep_announcements => "net.link.ether.inet.keep_announcements",
            SysctlKey::NetLinkEtherInetLog_arp_warnings => "net.link.ether.inet.log_arp_warnings",
            SysctlKey::NetLinkEtherInetMax_age => "net.link.ether.inet.max_age",
            SysctlKey::NetLinkEtherInetMaxhold => "net.link.ether.inet.maxhold",
            SysctlKey::NetLinkEtherInetMaxhold_total => "net.link.ether.inet.maxhold_total",
            SysctlKey::NetLinkEtherInetMaxtries => "net.link.ether.inet.maxtries",
            SysctlKey::NetLinkEtherInetProbe_intvl => "net.link.ether.inet.probe_intvl",
            SysctlKey::NetLinkEtherInetProxyall => "net.link.ether.inet.proxyall",
            SysctlKey::NetLinkEtherInetPrune_intvl => "net.link.ether.inet.prune_intvl",
            SysctlKey::NetLinkEtherInetSend_conflicting_probes => "net.link.ether.inet.send_conflicting_probes",
            SysctlKey::NetLinkEtherInetSendllconflict => "net.link.ether.inet.sendllconflict",
            SysctlKey::NetLinkEtherInetUseloopback => "net.link.ether.inet.useloopback",
            SysctlKey::NetLinkEtherInetVerbose => "net.link.ether.inet.verbose",
            SysctlKey::NetLinkFakeBsd_mode => "net.link.fake.bsd_mode",
            SysctlKey::NetLinkFakeBuflet_size => "net.link.fake.buflet_size",
            SysctlKey::NetLinkFakeCopypkt_mode => "net.link.fake.copypkt_mode",
            SysctlKey::NetLinkFakeDebug => "net.link.fake.debug",
            SysctlKey::NetLinkFakeHwcsum => "net.link.fake.hwcsum",
            SysctlKey::NetLinkFakeIf_adv_intvl => "net.link.fake.if_adv_intvl",
            SysctlKey::NetLinkFakeMax_mtu => "net.link.fake.max_mtu",
            SysctlKey::NetLinkFakeMultibuflet => "net.link.fake.multibuflet",
            SysctlKey::NetLinkFakeNxattach => "net.link.fake.nxattach",
            SysctlKey::NetLinkFakeTx_drops => "net.link.fake.tx_drops",
            SysctlKey::NetLinkFakeTx_headroom => "net.link.fake.tx_headroom",
            SysctlKey::NetLinkFakeTxstart => "net.link.fake.txstart",
            SysctlKey::NetLinkFakeUser_access => "net.link.fake.user_access",
            SysctlKey::NetLinkFakeWmm_mode => "net.link.fake.wmm_mode",
            SysctlKey::NetLinkGenericSystemDelaybased_queue => "net.link.generic.system.delaybased_queue",
            SysctlKey::NetLinkGenericSystemDlil_input_sanity_check => "net.link.generic.system.dlil_input_sanity_check",
            SysctlKey::NetLinkGenericSystemDlil_input_threads => "net.link.generic.system.dlil_input_threads",
            SysctlKey::NetLinkGenericSystemDlil_verbose => "net.link.generic.system.dlil_verbose",
            SysctlKey::NetLinkGenericSystemEnable_netagent => "net.link.generic.system.enable_netagent",
            SysctlKey::NetLinkGenericSystemFlow_advisory => "net.link.generic.system.flow_advisory",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg => "net.link.generic.system.hwcksum_dbg",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_adjusted => "net.link.generic.system.hwcksum_dbg_adjusted",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_bad_cksum => "net.link.generic.system.hwcksum_dbg_bad_cksum",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_bad_rxoff => "net.link.generic.system.hwcksum_dbg_bad_rxoff",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_finalized_data => "net.link.generic.system.hwcksum_dbg_finalized_data",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_finalized_hdr => "net.link.generic.system.hwcksum_dbg_finalized_hdr",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_mode => "net.link.generic.system.hwcksum_dbg_mode",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_forced => "net.link.generic.system.hwcksum_dbg_partial_forced",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_forced_bytes => "net.link.generic.system.hwcksum_dbg_partial_forced_bytes",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_rxoff_adj => "net.link.generic.system.hwcksum_dbg_partial_rxoff_adj",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_rxoff_forced => "net.link.generic.system.hwcksum_dbg_partial_rxoff_forced",
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_verified => "net.link.generic.system.hwcksum_dbg_verified",
            SysctlKey::NetLinkGenericSystemHwcksum_in_invalidated => "net.link.generic.system.hwcksum_in_invalidated",
            SysctlKey::NetLinkGenericSystemHwcksum_rx => "net.link.generic.system.hwcksum_rx",
            SysctlKey::NetLinkGenericSystemHwcksum_tx => "net.link.generic.system.hwcksum_tx",
            SysctlKey::NetLinkGenericSystemIf_verbose => "net.link.generic.system.if_verbose",
            SysctlKey::NetLinkGenericSystemIfcount => "net.link.generic.system.ifcount",
            SysctlKey::NetLinkGenericSystemPort_usedEntry_count => "net.link.generic.system.port_used.entry_count",
            SysctlKey::NetLinkGenericSystemPort_usedEntry_gen => "net.link.generic.system.port_used.entry_gen",
            SysctlKey::NetLinkGenericSystemPort_usedVerbose => "net.link.generic.system.port_used.verbose",
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_count => "net.link.generic.system.port_used.wakeuuid_not_set_count",
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_last_if => "net.link.generic.system.port_used.wakeuuid_not_set_last_if",
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_last_time => "net.link.generic.system.port_used.wakeuuid_not_set_last_time",
            SysctlKey::NetLinkGenericSystemRcvq_maxlen => "net.link.generic.system.rcvq_maxlen",
            SysctlKey::NetLinkGenericSystemRxpoll => "net.link.generic.system.rxpoll",
            SysctlKey::NetLinkGenericSystemRxpoll_decay => "net.link.generic.system.rxpoll_decay",
            SysctlKey::NetLinkGenericSystemRxpoll_freeze_time => "net.link.generic.system.rxpoll_freeze_time",
            SysctlKey::NetLinkGenericSystemRxpoll_interval_pkts => "net.link.generic.system.rxpoll_interval_pkts",
            SysctlKey::NetLinkGenericSystemRxpoll_interval_time => "net.link.generic.system.rxpoll_interval_time",
            SysctlKey::NetLinkGenericSystemRxpoll_max => "net.link.generic.system.rxpoll_max",
            SysctlKey::NetLinkGenericSystemRxpoll_sample_time => "net.link.generic.system.rxpoll_sample_time",
            SysctlKey::NetLinkGenericSystemRxpoll_wakeups_hiwat => "net.link.generic.system.rxpoll_wakeups_hiwat",
            SysctlKey::NetLinkGenericSystemRxpoll_wakeups_lowat => "net.link.generic.system.rxpoll_wakeups_lowat",
            SysctlKey::NetLinkGenericSystemSndq_maxlen => "net.link.generic.system.sndq_maxlen",
            SysctlKey::NetLinkGenericSystemStart_delay_disabled => "net.link.generic.system.start_delay_disabled",
            SysctlKey::NetLinkGenericSystemStart_delayed => "net.link.generic.system.start_delayed",
            SysctlKey::NetLinkGenericSystemThreshold_interval => "net.link.generic.system.threshold_interval",
            SysctlKey::NetLinkGenericSystemThreshold_notify => "net.link.generic.system.threshold_notify",
            SysctlKey::NetLinkGenericSystemTx_chain_len_count => "net.link.generic.system.tx_chain_len_count",
            SysctlKey::NetLinkIptapLog => "net.link.iptap.log",
            SysctlKey::NetLinkIptapTotal_tap_count => "net.link.iptap.total_tap_count",
            SysctlKey::NetLinkLoopbackDequeue_sc => "net.link.loopback.dequeue_sc",
            SysctlKey::NetLinkLoopbackMax_dequeue => "net.link.loopback.max_dequeue",
            SysctlKey::NetLinkLoopbackSched_model => "net.link.loopback.sched_model",
            SysctlKey::NetLinkPktapCount_unknown_if_type => "net.link.pktap.count_unknown_if_type",
            SysctlKey::NetLinkPktapLog => "net.link.pktap.log",
            SysctlKey::NetLinkPktapTotal_tap_count => "net.link.pktap.total_tap_count",
            SysctlKey::NetLocalDgramMaxdgram => "net.local.dgram.maxdgram",
            SysctlKey::NetLocalDgramRecvspace => "net.local.dgram.recvspace",
            SysctlKey::NetLocalInflight => "net.local.inflight",
            SysctlKey::NetLocalStreamRecvspace => "net.local.stream.recvspace",
            SysctlKey::NetLocalStreamSendspace => "net.local.stream.sendspace",
            SysctlKey::NetLocalStreamTracemdns => "net.local.stream.tracemdns",
            SysctlKey::NetMpklogEnabled => "net.mpklog.enabled",
            SysctlKey::NetMpklogType => "net.mpklog.type",
            SysctlKey::NetMpklogVersion => "net.mpklog.version",
            SysctlKey::NetNdrv_multi_max_count => "net.ndrv_multi_max_count",
            SysctlKey::NetNecpArena_count => "net.necp.arena_count",
            SysctlKey::NetNecpClient_count => "net.necp.client_count",
            SysctlKey::NetNecpClient_fd_count => "net.necp.client_fd_count",
            SysctlKey::NetNecpDebug => "net.necp.debug",
            SysctlKey::NetNecpDrop_all_level => "net.necp.drop_all_level",
            SysctlKey::NetNecpDrop_dest_debug => "net.necp.drop_dest_debug",
            SysctlKey::NetNecpDrop_unentitled_level => "net.necp.drop_unentitled_level",
            SysctlKey::NetNecpIf_flow_count => "net.necp.if_flow_count",
            SysctlKey::NetNecpIp_policy_count => "net.necp.ip_policy_count",
            SysctlKey::NetNecpNexus_flow_count => "net.necp.nexus_flow_count",
            SysctlKey::NetNecpObserver_fd_count => "net.necp.observer_fd_count",
            SysctlKey::NetNecpObserver_message_limit => "net.necp.observer_message_limit",
            SysctlKey::NetNecpPass_interpose => "net.necp.pass_interpose",
            SysctlKey::NetNecpPass_keepalives => "net.necp.pass_keepalives",
            SysctlKey::NetNecpPass_loopback => "net.necp.pass_loopback",
            SysctlKey::NetNecpSession_count => "net.necp.session_count",
            SysctlKey::NetNecpSocket_flow_count => "net.necp.socket_flow_count",
            SysctlKey::NetNecpSocket_non_app_policy_count => "net.necp.socket_non_app_policy_count",
            SysctlKey::NetNecpSocket_policy_count => "net.necp.socket_policy_count",
            SysctlKey::NetNecpSysctl_arena_count => "net.necp.sysctl_arena_count",
            SysctlKey::NetNetagentActive_count => "net.netagent.active_count",
            SysctlKey::NetNetagentDebug => "net.netagent.debug",
            SysctlKey::NetNetagentRegistered_count => "net.netagent.registered_count",
            SysctlKey::NetPktschedVerbose => "net.pktsched.verbose",
            SysctlKey::NetQosPolicyCapable_enabled => "net.qos.policy.capable_enabled",
            SysctlKey::NetQosPolicyRestrict_avapps => "net.qos.policy.restrict_avapps",
            SysctlKey::NetQosPolicyRestricted => "net.qos.policy.restricted",
            SysctlKey::NetQosPolicyWifi_enabled => "net.qos.policy.wifi_enabled",
            SysctlKey::NetQosReset_dscp_to_wifi_ac_map => "net.qos.reset_dscp_to_wifi_ac_map",
            SysctlKey::NetQosVerbose => "net.qos.verbose",
            SysctlKey::NetRestricted_portEnforced => "net.restricted_port.enforced",
            SysctlKey::NetRestricted_portVerbose => "net.restricted_port.verbose",
            SysctlKey::NetSmbFsKern_deadtimer => "net.smb.fs.kern_deadtimer",
            SysctlKey::NetSmbFsKern_hard_deadtimer => "net.smb.fs.kern_hard_deadtimer",
            SysctlKey::NetSmbFsKern_soft_deadtimer => "net.smb.fs.kern_soft_deadtimer",
            SysctlKey::NetSmbFsLoglevel => "net.smb.fs.loglevel",
            SysctlKey::NetSmbFsMaxread => "net.smb.fs.maxread",
            SysctlKey::NetSmbFsMaxsegreadsize => "net.smb.fs.maxsegreadsize",
            SysctlKey::NetSmbFsMaxsegwritesize => "net.smb.fs.maxsegwritesize",
            SysctlKey::NetSmbFsMaxwrite => "net.smb.fs.maxwrite",
            SysctlKey::NetSmbFsTcprcvbuf => "net.smb.fs.tcprcvbuf",
            SysctlKey::NetSmbFsTcpsndbuf => "net.smb.fs.tcpsndbuf",
            SysctlKey::NetSmbFsVersion => "net.smb.fs.version",
            SysctlKey::NetStatistics_privcheck => "net.statistics_privcheck",
            SysctlKey::NetStatsDebug => "net.stats.debug",
            SysctlKey::NetStatsRecvspace => "net.stats.recvspace",
            SysctlKey::NetStatsSendspace => "net.stats.sendspace",
            SysctlKey::NetSystmKctlAutorcvbufhigh => "net.systm.kctl.autorcvbufhigh",
            SysctlKey::NetSystmKctlAutorcvbufmax => "net.systm.kctl.autorcvbufmax",
            SysctlKey::NetSystmKctlDebug => "net.systm.kctl.debug",
            SysctlKey::NetUtunMax_pending_input => "net.utun.max_pending_input",
            SysctlKey::NetUtunRing_size => "net.utun.ring_size",
            SysctlKey::NetUtunRx_fsw_ring_size => "net.utun.rx_fsw_ring_size",
            SysctlKey::NetUtunTx_fsw_ring_size => "net.utun.tx_fsw_ring_size",
            SysctlKey::SecurityMacAmfiForce_policy => "security.mac.amfi.force_policy",
            SysctlKey::SecurityMacAmfiHsp_enable => "security.mac.amfi.hsp_enable",
            SysctlKey::SecurityMacAmfiVerbose_logging => "security.mac.amfi.verbose_logging",
            SysctlKey::SecurityMacAspActive_rule_version => "security.mac.asp.active_rule_version",
            SysctlKey::SecurityMacAspCache_allocation_count => "security.mac.asp.cache_allocation_count",
            SysctlKey::SecurityMacAspCache_entry_count => "security.mac.asp.cache_entry_count",
            SysctlKey::SecurityMacAspCache_release_count => "security.mac.asp.cache_release_count",
            SysctlKey::SecurityMacAspExec_hook_count => "security.mac.asp.exec_hook_count",
            SysctlKey::SecurityMacAspExec_hook_sleep_time => "security.mac.asp.exec_hook_sleep_time",
            SysctlKey::SecurityMacAspExec_hook_work_time => "security.mac.asp.exec_hook_work_time",
            SysctlKey::SecurityMacAspLibrary_hook_count => "security.mac.asp.library_hook_count",
            SysctlKey::SecurityMacAspLibrary_hook_time => "security.mac.asp.library_hook_time",
            SysctlKey::SecurityMacAspLibrary_sleep_time => "security.mac.asp.library_sleep_time",
            SysctlKey::SecurityMacAspPending_evaluation_count => "security.mac.asp.pending_evaluation_count",
            SysctlKey::SecurityMacDevice_enforce => "security.mac.device_enforce",
            SysctlKey::SecurityMacEndpointsecurityLog_level => "security.mac.endpointsecurity.log_level",
            SysctlKey::SecurityMacLabelvnodes => "security.mac.labelvnodes",
            SysctlKey::SecurityMacMax_slots => "security.mac.max_slots",
            SysctlKey::SecurityMacPipe_enforce => "security.mac.pipe_enforce",
            SysctlKey::SecurityMacPlatform_exec_logging => "security.mac.platform_exec_logging",
            SysctlKey::SecurityMacPosixsem_enforce => "security.mac.posixsem_enforce",
            SysctlKey::SecurityMacPosixshm_enforce => "security.mac.posixshm_enforce",
            SysctlKey::SecurityMacProc_enforce => "security.mac.proc_enforce",
            SysctlKey::SecurityMacQtnSandbox_enforce => "security.mac.qtn.sandbox_enforce",
            SysctlKey::SecurityMacQtnUser_approved_exec => "security.mac.qtn.user_approved_exec",
            SysctlKey::SecurityMacSandboxAudio_active => "security.mac.sandbox.audio_active",
            SysctlKey::SecurityMacSandboxSentinel => "security.mac.sandbox.sentinel",
            SysctlKey::SecurityMacSocket_enforce => "security.mac.socket_enforce",
            SysctlKey::SecurityMacSystem_enforce => "security.mac.system_enforce",
            SysctlKey::SecurityMacSysvmsg_enforce => "security.mac.sysvmsg_enforce",
            SysctlKey::SecurityMacSysvsem_enforce => "security.mac.sysvsem_enforce",
            SysctlKey::SecurityMacSysvshm_enforce => "security.mac.sysvshm_enforce",
            SysctlKey::SecurityMacVm_enforce => "security.mac.vm_enforce",
            SysctlKey::SecurityMacVnode_enforce => "security.mac.vnode_enforce",
            SysctlKey::SecurityMacVnode_label_count => "security.mac.vnode_label_count",
            SysctlKey::UserBc_base_max => "user.bc_base_max",
            SysctlKey::UserBc_dim_max => "user.bc_dim_max",
            SysctlKey::UserBc_scale_max => "user.bc_scale_max",
            SysctlKey::UserBc_string_max => "user.bc_string_max",
            SysctlKey::UserColl_weights_max => "user.coll_weights_max",
            SysctlKey::UserCs_path => "user.cs_path",
            SysctlKey::UserExpr_nest_max => "user.expr_nest_max",
            SysctlKey::UserLine_max => "user.line_max",
            SysctlKey::UserPosix2_c_bind => "user.posix2_c_bind",
            SysctlKey::UserPosix2_c_dev => "user.posix2_c_dev",
            SysctlKey::UserPosix2_char_term => "user.posix2_char_term",
            SysctlKey::UserPosix2_fort_dev => "user.posix2_fort_dev",
            SysctlKey::UserPosix2_fort_run => "user.posix2_fort_run",
            SysctlKey::UserPosix2_localedef => "user.posix2_localedef",
            SysctlKey::UserPosix2_sw_dev => "user.posix2_sw_dev",
            SysctlKey::UserPosix2_upe => "user.posix2_upe",
            SysctlKey::UserPosix2_version => "user.posix2_version",
            SysctlKey::UserRe_dup_max => "user.re_dup_max",
            SysctlKey::UserStream_max => "user.stream_max",
            SysctlKey::UserTzname_max => "user.tzname_max",
            SysctlKey::VfsGenericAlways_do_fullfsync => "vfs.generic.always_do_fullfsync",
            SysctlKey::VfsGenericApfsAllocated => "vfs.generic.apfs.allocated",
            SysctlKey::VfsGenericApfsFusion_elevator_throttle => "vfs.generic.apfs.fusion_elevator_throttle",
            SysctlKey::VfsGenericApfsFusion_lc_rc_promotion_threshold_mult => "vfs.generic.apfs.fusion_lc_rc_promotion_threshold_mult",
            SysctlKey::VfsGenericApfsFusion_paranoia_level => "vfs.generic.apfs.fusion_paranoia_level",
            SysctlKey::VfsGenericApfsFusion_promoter_queue_limit => "vfs.generic.apfs.fusion_promoter_queue_limit",
            SysctlKey::VfsGenericApfsFusion_promoter_throttle => "vfs.generic.apfs.fusion_promoter_throttle",
            SysctlKey::VfsGenericApfsFusion_rc_flags => "vfs.generic.apfs.fusion_rc_flags",
            SysctlKey::VfsGenericApfsFusion_rc_promotion_size_limit_mb => "vfs.generic.apfs.fusion_rc_promotion_size_limit_mb",
            SysctlKey::VfsGenericApfsFusion_rc_promotion_threshold_mult => "vfs.generic.apfs.fusion_rc_promotion_threshold_mult",
            SysctlKey::VfsGenericApfsFusion_slow_io_threshold => "vfs.generic.apfs.fusion_slow_io_threshold",
            SysctlKey::VfsGenericApfsFusion_swapfile_backoff => "vfs.generic.apfs.fusion_swapfile_backoff",
            SysctlKey::VfsGenericApfsFusion_verbosity_flags => "vfs.generic.apfs.fusion_verbosity_flags",
            SysctlKey::VfsGenericApfsFusion_w2rc_filled_ratio_threshold => "vfs.generic.apfs.fusion_w2rc_filled_ratio_threshold",
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_low => "vfs.generic.apfs.fusion_wbc_backoff_wmk_low",
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_med => "vfs.generic.apfs.fusion_wbc_backoff_wmk_med",
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_reenable => "vfs.generic.apfs.fusion_wbc_backoff_wmk_reenable",
            SysctlKey::VfsGenericApfsFusion_wbc_buffersize => "vfs.generic.apfs.fusion_wbc_buffersize",
            SysctlKey::VfsGenericApfsFusion_wbc_elevator_wmk => "vfs.generic.apfs.fusion_wbc_elevator_wmk",
            SysctlKey::VfsGenericAutofsVnode_recycle_on_inactive => "vfs.generic.autofs.vnode_recycle_on_inactive",
            SysctlKey::VfsGenericHfsAllocated => "vfs.generic.hfs.allocated",
            SysctlKey::VfsGenericHfsJnlKdebugTrim => "vfs.generic.hfs.jnl.kdebug.trim",
            SysctlKey::VfsGenericHfsJnlTrim_flush => "vfs.generic.hfs.jnl.trim_flush",
            SysctlKey::VfsGenericHfsKdebugAllocation => "vfs.generic.hfs.kdebug.allocation",
            SysctlKey::VfsGenericMaxtypenum => "vfs.generic.maxtypenum",
            SysctlKey::VfsGenericNfsClientAccess_cache_timeout => "vfs.generic.nfs.client.access_cache_timeout",
            SysctlKey::VfsGenericNfsClientAccess_delete => "vfs.generic.nfs.client.access_delete",
            SysctlKey::VfsGenericNfsClientAccess_dotzfs => "vfs.generic.nfs.client.access_dotzfs",
            SysctlKey::VfsGenericNfsClientAccess_for_getattr => "vfs.generic.nfs.client.access_for_getattr",
            SysctlKey::VfsGenericNfsClientAllow_async => "vfs.generic.nfs.client.allow_async",
            SysctlKey::VfsGenericNfsClientCallback_port => "vfs.generic.nfs.client.callback_port",
            SysctlKey::VfsGenericNfsClientDebug_ctl => "vfs.generic.nfs.client.debug_ctl",
            SysctlKey::VfsGenericNfsClientDefault_nfs4domain => "vfs.generic.nfs.client.default_nfs4domain",
            SysctlKey::VfsGenericNfsClientIdmap_ctrl => "vfs.generic.nfs.client.idmap_ctrl",
            SysctlKey::VfsGenericNfsClientInitialdowndelay => "vfs.generic.nfs.client.initialdowndelay",
            SysctlKey::VfsGenericNfsClientIosize => "vfs.generic.nfs.client.iosize",
            SysctlKey::VfsGenericNfsClientIs_mobile => "vfs.generic.nfs.client.is_mobile",
            SysctlKey::VfsGenericNfsClientLockd_mounts => "vfs.generic.nfs.client.lockd_mounts",
            SysctlKey::VfsGenericNfsClientMax_async_writes => "vfs.generic.nfs.client.max_async_writes",
            SysctlKey::VfsGenericNfsClientNextdowndelay => "vfs.generic.nfs.client.nextdowndelay",
            SysctlKey::VfsGenericNfsClientNfsiod_thread_count => "vfs.generic.nfs.client.nfsiod_thread_count",
            SysctlKey::VfsGenericNfsClientNfsiod_thread_max => "vfs.generic.nfs.client.nfsiod_thread_max",
            SysctlKey::VfsGenericNfsClientReadlink_nocache => "vfs.generic.nfs.client.readlink_nocache",
            SysctlKey::VfsGenericNfsClientRoot_steals_gss_context => "vfs.generic.nfs.client.root_steals_gss_context",
            SysctlKey::VfsGenericNfsClientSquishy_flags => "vfs.generic.nfs.client.squishy_flags",
            SysctlKey::VfsGenericNfsClientStatfs_rate_limit => "vfs.generic.nfs.client.statfs_rate_limit",
            SysctlKey::VfsGenericNfsServerAsync => "vfs.generic.nfs.server.async",
            SysctlKey::VfsGenericNfsServerExport_hash_size => "vfs.generic.nfs.server.export_hash_size",
            SysctlKey::VfsGenericNfsServerFsevents => "vfs.generic.nfs.server.fsevents",
            SysctlKey::VfsGenericNfsServerGss_context_ttl => "vfs.generic.nfs.server.gss_context_ttl",
            SysctlKey::VfsGenericNfsServerNfsd_sock_idle_timeout => "vfs.generic.nfs.server.nfsd_sock_idle_timeout",
            SysctlKey::VfsGenericNfsServerNfsd_tcp_connections => "vfs.generic.nfs.server.nfsd_tcp_connections",
            SysctlKey::VfsGenericNfsServerNfsd_thread_count => "vfs.generic.nfs.server.nfsd_thread_count",
            SysctlKey::VfsGenericNfsServerNfsd_thread_max => "vfs.generic.nfs.server.nfsd_thread_max",
            SysctlKey::VfsGenericNfsServerReqcache_size => "vfs.generic.nfs.server.reqcache_size",
            SysctlKey::VfsGenericNfsServerRequest_queue_length => "vfs.generic.nfs.server.request_queue_length",
            SysctlKey::VfsGenericNfsServerRequire_resv_port => "vfs.generic.nfs.server.require_resv_port",
            SysctlKey::VfsGenericNfsServerUpcall_queue_count => "vfs.generic.nfs.server.upcall_queue_count",
            SysctlKey::VfsGenericNfsServerUpcall_queue_limit => "vfs.generic.nfs.server.upcall_queue_limit",
            SysctlKey::VfsGenericNfsServerUpcall_queue_max_seen => "vfs.generic.nfs.server.upcall_queue_max_seen",
            SysctlKey::VfsGenericNfsServerUse_upcall_svc => "vfs.generic.nfs.server.use_upcall_svc",
            SysctlKey::VfsGenericNfsServerUser_stats => "vfs.generic.nfs.server.user_stats",
            SysctlKey::VfsGenericNfsServerWg_delay => "vfs.generic.nfs.server.wg_delay",
            SysctlKey::VfsGenericNfsServerWg_delay_v3 => "vfs.generic.nfs.server.wg_delay_v3",
            SysctlKey::VfsGenericRoot_unmounted_cleanly => "vfs.generic.root_unmounted_cleanly",
            SysctlKey::VfsGenericSync_timeout => "vfs.generic.sync_timeout",
            SysctlKey::VfsNspacePrevent_materialization => "vfs.nspace.prevent_materialization",
            SysctlKey::VfsNspaceResolver => "vfs.nspace.resolver",
            SysctlKey::VfsNspaceThread_prevent_materialization => "vfs.nspace.thread_prevent_materialization",
            SysctlKey::VfsNummntops => "vfs.nummntops",
            SysctlKey::VmAll_reusable_calls => "vm.all_reusable_calls",
            SysctlKey::VmAll_reuse_calls => "vm.all_reuse_calls",
            SysctlKey::VmCan_reuse_failure => "vm.can_reuse_failure",
            SysctlKey::VmCan_reuse_success => "vm.can_reuse_success",
            SysctlKey::VmCompressor_available => "vm.compressor_available",
            SysctlKey::VmCompressor_bytes_used => "vm.compressor_bytes_used",
            SysctlKey::VmCompressor_compressed_bytes => "vm.compressor_compressed_bytes",
            SysctlKey::VmCompressor_eval_period_in_msecs => "vm.compressor_eval_period_in_msecs",
            SysctlKey::VmCompressor_input_bytes => "vm.compressor_input_bytes",
            SysctlKey::VmCompressor_is_active => "vm.compressor_is_active",
            SysctlKey::VmCompressor_min_csegs_per_major_compaction => "vm.compressor_min_csegs_per_major_compaction",
            SysctlKey::VmCompressor_mode => "vm.compressor_mode",
            SysctlKey::VmCompressor_sample_max_in_msecs => "vm.compressor_sample_max_in_msecs",
            SysctlKey::VmCompressor_sample_min_in_msecs => "vm.compressor_sample_min_in_msecs",
            SysctlKey::VmCompressor_swapout_target_age => "vm.compressor_swapout_target_age",
            SysctlKey::VmCompressor_thrashing_min_per_10msecs => "vm.compressor_thrashing_min_per_10msecs",
            SysctlKey::VmCompressor_thrashing_threshold_per_10msecs => "vm.compressor_thrashing_threshold_per_10msecs",
            SysctlKey::VmCompressor_timing_enabled => "vm.compressor_timing_enabled",
            SysctlKey::VmCopied_on_read => "vm.copied_on_read",
            SysctlKey::VmCorpse_footprint_count => "vm.corpse_footprint_count",
            SysctlKey::VmCorpse_footprint_full => "vm.corpse_footprint_full",
            SysctlKey::VmCorpse_footprint_no_buf => "vm.corpse_footprint_no_buf",
            SysctlKey::VmCorpse_footprint_size_avg => "vm.corpse_footprint_size_avg",
            SysctlKey::VmCorpse_footprint_size_max => "vm.corpse_footprint_size_max",
            SysctlKey::VmCs_all_vnodes => "vm.cs_all_vnodes",
            SysctlKey::VmCs_blob_count => "vm.cs_blob_count",
            SysctlKey::VmCs_blob_count_peak => "vm.cs_blob_count_peak",
            SysctlKey::VmCs_blob_size => "vm.cs_blob_size",
            SysctlKey::VmCs_blob_size_max => "vm.cs_blob_size_max",
            SysctlKey::VmCs_blob_size_peak => "vm.cs_blob_size_peak",
            SysctlKey::VmCs_debug => "vm.cs_debug",
            SysctlKey::VmCs_debug_fail_on_unsigned_code => "vm.cs_debug_fail_on_unsigned_code",
            SysctlKey::VmCs_debug_unsigned_exec_failures => "vm.cs_debug_unsigned_exec_failures",
            SysctlKey::VmCs_debug_unsigned_mmap_failures => "vm.cs_debug_unsigned_mmap_failures",
            SysctlKey::VmCs_enforcement_panic => "vm.cs_enforcement_panic",
            SysctlKey::VmCs_force_hard => "vm.cs_force_hard",
            SysctlKey::VmCs_force_kill => "vm.cs_force_kill",
            SysctlKey::VmCs_library_validation => "vm.cs_library_validation",
            SysctlKey::VmCs_process_enforcement => "vm.cs_process_enforcement",
            SysctlKey::VmCs_system_enforcement => "vm.cs_system_enforcement",
            SysctlKey::VmDarkwake_mode => "vm.darkwake_mode",
            SysctlKey::VmFree_shared => "vm.free_shared",
            SysctlKey::VmGlobal_no_user_wire_amount => "vm.global_no_user_wire_amount",
            SysctlKey::VmGlobal_user_wire_limit => "vm.global_user_wire_limit",
            SysctlKey::VmIopl_pages_tainted => "vm.iopl_pages_tainted",
            SysctlKey::VmKern_lpage_count => "vm.kern_lpage_count",
            SysctlKey::VmLoadavg => "vm.loadavg",
            SysctlKey::VmLz4_compressed_bytes => "vm.lz4_compressed_bytes",
            SysctlKey::VmLz4_compression_failures => "vm.lz4_compression_failures",
            SysctlKey::VmLz4_compressions => "vm.lz4_compressions",
            SysctlKey::VmLz4_decompressed_bytes => "vm.lz4_decompressed_bytes",
            SysctlKey::VmLz4_decompressions => "vm.lz4_decompressions",
            SysctlKey::VmLz4_max_failure_run_length => "vm.lz4_max_failure_run_length",
            SysctlKey::VmLz4_max_failure_skips => "vm.lz4_max_failure_skips",
            SysctlKey::VmLz4_max_preselects => "vm.lz4_max_preselects",
            SysctlKey::VmLz4_profitable_bytes => "vm.lz4_profitable_bytes",
            SysctlKey::VmLz4_run_continue_bytes => "vm.lz4_run_continue_bytes",
            SysctlKey::VmLz4_run_preselection_threshold => "vm.lz4_run_preselection_threshold",
            SysctlKey::VmLz4_threshold => "vm.lz4_threshold",
            SysctlKey::VmLz4_wk_compression_delta => "vm.lz4_wk_compression_delta",
            SysctlKey::VmLz4_wk_compression_negative_delta => "vm.lz4_wk_compression_negative_delta",
            SysctlKey::VmMadvise_free_debug => "vm.madvise_free_debug",
            SysctlKey::VmMemory_pressure => "vm.memory_pressure",
            SysctlKey::VmPage_busy_absent_skipped => "vm.page_busy_absent_skipped",
            SysctlKey::VmPage_cleaned_count => "vm.page_cleaned_count",
            SysctlKey::VmPage_free_count => "vm.page_free_count",
            SysctlKey::VmPage_free_wanted => "vm.page_free_wanted",
            SysctlKey::VmPage_pageable_external_count => "vm.page_pageable_external_count",
            SysctlKey::VmPage_pageable_internal_count => "vm.page_pageable_internal_count",
            SysctlKey::VmPage_purgeable_count => "vm.page_purgeable_count",
            SysctlKey::VmPage_purgeable_wired_count => "vm.page_purgeable_wired_count",
            SysctlKey::VmPage_reusable_count => "vm.page_reusable_count",
            SysctlKey::VmPage_speculative_count => "vm.page_speculative_count",
            SysctlKey::VmPageout_freed_cleaned => "vm.pageout_freed_cleaned",
            SysctlKey::VmPageout_freed_external => "vm.pageout_freed_external",
            SysctlKey::VmPageout_freed_speculative => "vm.pageout_freed_speculative",
            SysctlKey::VmPageout_inactive_clean => "vm.pageout_inactive_clean",
            SysctlKey::VmPageout_inactive_dirty_external => "vm.pageout_inactive_dirty_external",
            SysctlKey::VmPageout_inactive_dirty_internal => "vm.pageout_inactive_dirty_internal",
            SysctlKey::VmPageout_inactive_used => "vm.pageout_inactive_used",
            SysctlKey::VmPageout_speculative_clean => "vm.pageout_speculative_clean",
            SysctlKey::VmPages => "vm.pages",
            SysctlKey::VmPagesize => "vm.pagesize",
            SysctlKey::VmPartial_reusable_calls => "vm.partial_reusable_calls",
            SysctlKey::VmPartial_reuse_calls => "vm.partial_reuse_calls",
            SysctlKey::VmPrefault_nb_bailout => "vm.prefault_nb_bailout",
            SysctlKey::VmPrefault_nb_pages => "vm.prefault_nb_pages",
            SysctlKey::VmProtect_privileged_from_untrusted => "vm.protect_privileged_from_untrusted",
            SysctlKey::VmReusable_failure => "vm.reusable_failure",
            SysctlKey::VmReusable_nonwritable => "vm.reusable_nonwritable",
            SysctlKey::VmReusable_pages_shared => "vm.reusable_pages_shared",
            SysctlKey::VmReusable_reclaimed => "vm.reusable_reclaimed",
            SysctlKey::VmReusable_shared => "vm.reusable_shared",
            SysctlKey::VmReusable_success => "vm.reusable_success",
            SysctlKey::VmReuse_failure => "vm.reuse_failure",
            SysctlKey::VmReuse_success => "vm.reuse_success",
            SysctlKey::VmShared_region_pager_copied => "vm.shared_region_pager_copied",
            SysctlKey::VmShared_region_pager_reclaimed => "vm.shared_region_pager_reclaimed",
            SysctlKey::VmShared_region_pager_slid => "vm.shared_region_pager_slid",
            SysctlKey::VmShared_region_pager_slid_error => "vm.shared_region_pager_slid_error",
            SysctlKey::VmShared_region_persistence => "vm.shared_region_persistence",
            SysctlKey::VmShared_region_trace_level => "vm.shared_region_trace_level",
            SysctlKey::VmShared_region_unnest_logging => "vm.shared_region_unnest_logging",
            SysctlKey::VmShared_region_version => "vm.shared_region_version",
            SysctlKey::VmSwapfileprefix => "vm.swapfileprefix",
            SysctlKey::VmSwapusage => "vm.swapusage",
            SysctlKey::VmUc_decompressions => "vm.uc_decompressions",
            SysctlKey::VmUpl_pages_tainted => "vm.upl_pages_tainted",
            SysctlKey::VmUser_wire_limit => "vm.user_wire_limit",
            SysctlKey::VmVm_clump_promote_threshold => "vm.vm_clump_promote_threshold",
            SysctlKey::VmVm_copy_src_large => "vm.vm_copy_src_large",
            SysctlKey::VmVm_copy_src_not_internal => "vm.vm_copy_src_not_internal",
            SysctlKey::VmVm_copy_src_not_symmetric => "vm.vm_copy_src_not_symmetric",
            SysctlKey::VmVm_create_upl_extra_cow => "vm.vm_create_upl_extra_cow",
            SysctlKey::VmVm_create_upl_extra_cow_pages => "vm.vm_create_upl_extra_cow_pages",
            SysctlKey::VmVm_create_upl_lookup_failure_copy => "vm.vm_create_upl_lookup_failure_copy",
            SysctlKey::VmVm_create_upl_lookup_failure_write => "vm.vm_create_upl_lookup_failure_write",
            SysctlKey::VmVm_debug_events => "vm.vm_debug_events",
            SysctlKey::VmVm_do_collapse_compressor => "vm.vm_do_collapse_compressor",
            SysctlKey::VmVm_do_collapse_compressor_pages => "vm.vm_do_collapse_compressor_pages",
            SysctlKey::VmVm_do_collapse_terminate => "vm.vm_do_collapse_terminate",
            SysctlKey::VmVm_do_collapse_terminate_failure => "vm.vm_do_collapse_terminate_failure",
            SysctlKey::VmVm_max_kernel_address => "vm.vm_max_kernel_address",
            SysctlKey::VmVm_min_kernel_address => "vm.vm_min_kernel_address",
            SysctlKey::VmVm_page_background_count => "vm.vm_page_background_count",
            SysctlKey::VmVm_page_background_exclude_external => "vm.vm_page_background_exclude_external",
            SysctlKey::VmVm_page_background_external_count => "vm.vm_page_background_external_count",
            SysctlKey::VmVm_page_background_internal_count => "vm.vm_page_background_internal_count",
            SysctlKey::VmVm_page_background_mode => "vm.vm_page_background_mode",
            SysctlKey::VmVm_page_background_promoted_count => "vm.vm_page_background_promoted_count",
            SysctlKey::VmVm_page_background_target => "vm.vm_page_background_target",
            SysctlKey::VmVm_page_external_count => "vm.vm_page_external_count",
            SysctlKey::VmVm_page_filecache_min => "vm.vm_page_filecache_min",
            SysctlKey::VmVm_page_free_target => "vm.vm_page_free_target",
            SysctlKey::VmVm_page_xpmapped_min => "vm.vm_page_xpmapped_min",
            SysctlKey::VmVm_pageout_considered_bq_external => "vm.vm_pageout_considered_bq_external",
            SysctlKey::VmVm_pageout_considered_bq_internal => "vm.vm_pageout_considered_bq_internal",
            SysctlKey::VmVm_pageout_rejected_bq_external => "vm.vm_pageout_rejected_bq_external",
            SysctlKey::VmVm_pageout_rejected_bq_internal => "vm.vm_pageout_rejected_bq_internal",
            SysctlKey::VmVm_ripe_target_age_in_secs => "vm.vm_ripe_target_age_in_secs",
            SysctlKey::VmVm_should_cow_but_wired => "vm.vm_should_cow_but_wired",
            SysctlKey::VmWk_catime => "vm.wk_catime",
            SysctlKey::VmWk_compressed_bytes_exclusive => "vm.wk_compressed_bytes_exclusive",
            SysctlKey::VmWk_compressed_bytes_total => "vm.wk_compressed_bytes_total",
            SysctlKey::VmWk_compression_failures => "vm.wk_compression_failures",
            SysctlKey::VmWk_compressions => "vm.wk_compressions",
            SysctlKey::VmWk_compressions_exclusive => "vm.wk_compressions_exclusive",
            SysctlKey::VmWk_datime => "vm.wk_datime",
            SysctlKey::VmWk_decompressed_bytes => "vm.wk_decompressed_bytes",
            SysctlKey::VmWk_decompressions => "vm.wk_decompressions",
            SysctlKey::VmWk_mzv_compressions => "vm.wk_mzv_compressions",
            SysctlKey::VmWk_sv_compressions => "vm.wk_sv_compressions",
            SysctlKey::VmWk_sv_decompressions => "vm.wk_sv_decompressions",
            SysctlKey::VmWkdm_reeval_threshold => "vm.wkdm_reeval_threshold",
            SysctlKey::VmWkh_catime => "vm.wkh_catime",
            SysctlKey::VmWkh_compressions => "vm.wkh_compressions",
            SysctlKey::VmWkh_datime => "vm.wkh_datime",
            SysctlKey::VmWkh_decompressions => "vm.wkh_decompressions",
            SysctlKey::VmWks_catime => "vm.wks_catime",
            SysctlKey::VmWks_compressed_bytes => "vm.wks_compressed_bytes",
            SysctlKey::VmWks_compression_failures => "vm.wks_compression_failures",
            SysctlKey::VmWks_compressions => "vm.wks_compressions",
            SysctlKey::VmWks_datime => "vm.wks_datime",
            SysctlKey::VmWks_decompressions => "vm.wks_decompressions",
            SysctlKey::VmWks_sv_compressions => "vm.wks_sv_compressions",
        }
    }
}
impl SysctlKey {
    /// Returns a vector containing all Enum elements.
    pub fn list() -> Vec<SysctlKey> {
        vec![
            SysctlKey::AuditSessionMember_clear_sflags_mask,
            SysctlKey::AuditSessionMember_set_sflags_mask,
            SysctlKey::AuditSessionSuperuser_clear_sflags_mask,
            SysctlKey::AuditSessionSuperuser_set_sflags_mask,
            SysctlKey::DebugAcpi_flags,
            SysctlKey::DebugAcpi_layer,
            SysctlKey::DebugAcpi_level,
            SysctlKey::DebugAgpmLogLevel,
            SysctlKey::DebugBatman,
            SysctlKey::DebugBpf_bufsize,
            SysctlKey::DebugBpf_debug,
            SysctlKey::DebugBpf_maxbufsize,
            SysctlKey::DebugBpf_maxdevices,
            SysctlKey::DebugBpf_wantpktap,
            SysctlKey::DebugBrcmfirewirelog,
            SysctlKey::DebugBrcmlinkdebug,
            SysctlKey::DebugBrcmlogging,
            SysctlKey::DebugDarkwake,
            SysctlKey::DebugIntelDtraceEnable,
            SysctlKey::DebugIntelFlipCount,
            SysctlKey::DebugIntelGlobalUsageTotal_Busy_nSec,
            SysctlKey::DebugIntelGlobalUsageTotal_nSec,
            SysctlKey::DebugIntelGpuUsageEnables,
            SysctlKey::DebugIntelGpuUsageEnablesCheck,
            SysctlKey::DebugIntelGraphicsTracePointEnable,
            SysctlKey::DebugIntelIGInterruptControl,
            SysctlKey::DebugIntelKdctlVersion,
            SysctlKey::DebugIntelMSecCalcGPUBusy,
            SysctlKey::DebugIntelOaEnable,
            SysctlKey::DebugIntelPerfEventEnable,
            SysctlKey::DebugIntelRingBlitUsage,
            SysctlKey::DebugIntelRingBlit_nSec,
            SysctlKey::DebugIntelRingMainUsage,
            SysctlKey::DebugIntelRingMain_nSec,
            SysctlKey::DebugIntelRingMediaUsage,
            SysctlKey::DebugIntelRingMedia_nSec,
            SysctlKey::DebugIntelRingOnSample,
            SysctlKey::DebugIntelRingTakeSample,
            SysctlKey::DebugIntelRingVEBoxUsage,
            SysctlKey::DebugIntelRingVEBox_nSec,
            SysctlKey::DebugIntelSchedEnableThrottleOverride,
            SysctlKey::DebugIntelSchedPriCreditsHigh,
            SysctlKey::DebugIntelSchedPriCreditsLow,
            SysctlKey::DebugIntelSchedPriCreditsNormal,
            SysctlKey::DebugIntelSchedPriCreditsNormalHigh,
            SysctlKey::DebugIntelSchedPriElevatePID,
            SysctlKey::DebugIntelSchedPriPreemption,
            SysctlKey::DebugIntelSchedThrottleHighPriByVal,
            SysctlKey::DebugIntelSchedThrottleLowPriByVal,
            SysctlKey::DebugIntelSchedThrottleNormalHighPriByVal,
            SysctlKey::DebugIntelSchedThrottleNormalPriByVal,
            SysctlKey::DebugIntelSwapCount,
            SysctlKey::DebugIntelTelemetryAltConfig,
            SysctlKey::DebugIntelTelemetryConfig,
            SysctlKey::DebugIntelTelemetryMode,
            SysctlKey::DebugIntelTelemetryNumFrame,
            SysctlKey::DebugIntelTelemetrySampleLocations,
            SysctlKey::DebugIntelTelemetrySpot1,
            SysctlKey::DebugIntelTelemetryStartFrame,
            SysctlKey::DebugIntelTelemetryStatPasses,
            SysctlKey::DebugIntelTelemetryStopFrame,
            SysctlKey::DebugIntelTelemetryTestCase,
            SysctlKey::DebugIntelTelemetryUsageReportmSec,
            SysctlKey::DebugIntelTelemetryVersion,
            SysctlKey::DebugIntelTemp0,
            SysctlKey::DebugIntelTemp1,
            SysctlKey::DebugIntelTemp2,
            SysctlKey::DebugIntelTemp3,
            SysctlKey::DebugIntelTemp4,
            SysctlKey::DebugIntelfbEUCount,
            SysctlKey::DebugIntelfbFLastRequestedPState,
            SysctlKey::DebugIntelfbFakeType2Dongle,
            SysctlKey::DebugIntelfbForceSlicesGTx,
            SysctlKey::DebugIntelfbGraphicsTracePointEnable,
            SysctlKey::DebugIntelfbIGInterruptControl,
            SysctlKey::DebugIntelfbSliceInfo,
            SysctlKey::DebugIntelfbTemp0,
            SysctlKey::DebugIntelfbTemp1,
            SysctlKey::DebugIntelfbTemp2,
            SysctlKey::DebugIntelfbTemp3,
            SysctlKey::DebugIntelfbTemp4,
            SysctlKey::DebugIntelfbTestCase,
            SysctlKey::DebugIokit,
            SysctlKey::DebugIoppf,
            SysctlKey::DebugIotrace,
            SysctlKey::DebugKextlog,
            SysctlKey::DebugLowpri_throttle_enabled,
            SysctlKey::DebugLowpri_throttle_max_iosize,
            SysctlKey::DebugLowpri_throttle_tier1_io_period_msecs,
            SysctlKey::DebugLowpri_throttle_tier1_io_period_ssd_msecs,
            SysctlKey::DebugLowpri_throttle_tier1_window_msecs,
            SysctlKey::DebugLowpri_throttle_tier2_io_period_msecs,
            SysctlKey::DebugLowpri_throttle_tier2_io_period_ssd_msecs,
            SysctlKey::DebugLowpri_throttle_tier2_window_msecs,
            SysctlKey::DebugLowpri_throttle_tier3_io_period_msecs,
            SysctlKey::DebugLowpri_throttle_tier3_io_period_ssd_msecs,
            SysctlKey::DebugLowpri_throttle_tier3_window_msecs,
            SysctlKey::DebugNoidle,
            SysctlKey::DebugSched,
            SysctlKey::DebugSwd_panic,
            SysctlKey::DebugSwd_sleep_timeout,
            SysctlKey::DebugSwd_timeout,
            SysctlKey::DebugSwd_wake_timeout,
            SysctlKey::DebugToggle_address_reuse,
            SysctlKey::DebugWlan_ll_debug,
            SysctlKey::HwActivecpu,
            SysctlKey::HwBusfrequency,
            SysctlKey::HwBusfrequency_max,
            SysctlKey::HwBusfrequency_min,
            SysctlKey::HwByteorder,
            SysctlKey::HwCacheconfig,
            SysctlKey::HwCachelinesize,
            SysctlKey::HwCachesize,
            SysctlKey::HwCpu64bit_capable,
            SysctlKey::HwCpufamily,
            SysctlKey::HwCpufrequency,
            SysctlKey::HwCpufrequency_max,
            SysctlKey::HwCpufrequency_min,
            SysctlKey::HwCpusubtype,
            SysctlKey::HwCputhreadtype,
            SysctlKey::HwCputype,
            SysctlKey::HwL1dcachesize,
            SysctlKey::HwL1icachesize,
            SysctlKey::HwL2cachesize,
            SysctlKey::HwL3cachesize,
            SysctlKey::HwLogicalcpu,
            SysctlKey::HwLogicalcpu_max,
            SysctlKey::HwMemsize,
            SysctlKey::HwNcpu,
            SysctlKey::HwOptionalAdx,
            SysctlKey::HwOptionalAes,
            SysctlKey::HwOptionalAvx1_0,
            SysctlKey::HwOptionalAvx2_0,
            SysctlKey::HwOptionalAvx512bw,
            SysctlKey::HwOptionalAvx512cd,
            SysctlKey::HwOptionalAvx512dq,
            SysctlKey::HwOptionalAvx512f,
            SysctlKey::HwOptionalAvx512ifma,
            SysctlKey::HwOptionalAvx512vbmi,
            SysctlKey::HwOptionalAvx512vl,
            SysctlKey::HwOptionalBmi1,
            SysctlKey::HwOptionalBmi2,
            SysctlKey::HwOptionalEnfstrg,
            SysctlKey::HwOptionalF16c,
            SysctlKey::HwOptionalFloatingpoint,
            SysctlKey::HwOptionalFma,
            SysctlKey::HwOptionalHle,
            SysctlKey::HwOptionalMmx,
            SysctlKey::HwOptionalMpx,
            SysctlKey::HwOptionalRdrand,
            SysctlKey::HwOptionalRtm,
            SysctlKey::HwOptionalSgx,
            SysctlKey::HwOptionalSse,
            SysctlKey::HwOptionalSse2,
            SysctlKey::HwOptionalSse3,
            SysctlKey::HwOptionalSse4_1,
            SysctlKey::HwOptionalSse4_2,
            SysctlKey::HwOptionalSupplementalsse3,
            SysctlKey::HwOptionalX86_64,
            SysctlKey::HwPackages,
            SysctlKey::HwPagesize,
            SysctlKey::HwPagesize32,
            SysctlKey::HwPhysicalcpu,
            SysctlKey::HwPhysicalcpu_max,
            SysctlKey::HwTargettype,
            SysctlKey::HwTbfrequency,
            SysctlKey::KernAffinity_sets_enabled,
            SysctlKey::KernAffinity_sets_mapping,
            SysctlKey::KernAiomax,
            SysctlKey::KernAioprocmax,
            SysctlKey::KernAiothreads,
            SysctlKey::KernAotmode,
            SysctlKey::KernAotmodebits,
            SysctlKey::KernArgmax,
            SysctlKey::KernBootargs,
            SysctlKey::KernBootsessionuuid,
            SysctlKey::KernBootsignature,
            SysctlKey::KernBoottime,
            SysctlKey::KernCheck_openevt,
            SysctlKey::KernClockrate,
            SysctlKey::KernConsoleoptions,
            SysctlKey::KernCoredump,
            SysctlKey::KernCorefile,
            SysctlKey::KernDelayterm,
            SysctlKey::KernDs_supgroups_supported,
            SysctlKey::KernDtraceBuffer_memory_inuse,
            SysctlKey::KernDtraceBuffer_memory_maxsize,
            SysctlKey::KernDtraceDifo_maxsize,
            SysctlKey::KernDtraceDof_maxsize,
            SysctlKey::KernDtraceDof_mode,
            SysctlKey::KernDtraceErr_verbose,
            SysctlKey::KernDtraceGlobal_maxsize,
            SysctlKey::KernDtraceIgnore_fbt_blacklist,
            SysctlKey::KernDtraceProvide_private_probes,
            SysctlKey::KernEventhandlerDebug,
            SysctlKey::KernFlush_cache_on_write,
            SysctlKey::KernHibernatefile,
            SysctlKey::KernHibernategraphicsready,
            SysctlKey::KernHibernatehidready,
            SysctlKey::KernHibernatelockscreenready,
            SysctlKey::KernHibernatemode,
            SysctlKey::KernHibernatewakenotification,
            SysctlKey::KernHostid,
            SysctlKey::KernHostname,
            SysctlKey::KernHvVmx_mitigations,
            SysctlKey::KernHvVmx_supported_mitigations,
            SysctlKey::KernHv_support,
            SysctlKey::KernInterrupt_timer_coalescing_enabled,
            SysctlKey::KernIokittest,
            SysctlKey::KernIossupportversion,
            SysctlKey::KernIpcExtbkidlercvhiwat,
            SysctlKey::KernIpcExtbkidletime,
            SysctlKey::KernIpcIo_policyLog,
            SysctlKey::KernIpcIo_policyUuid,
            SysctlKey::KernIpcMaxextbkidleperproc,
            SysctlKey::KernIpcMaxrecvmsgx,
            SysctlKey::KernIpcMaxsendmsgx,
            SysctlKey::KernIpcMaxsockbuf,
            SysctlKey::KernIpcMb_drain_force,
            SysctlKey::KernIpcMb_drain_maxint,
            SysctlKey::KernIpcMb_normalized,
            SysctlKey::KernIpcMb_watchdog,
            SysctlKey::KernIpcMleak_sample_factor,
            SysctlKey::KernIpcNjcl,
            SysctlKey::KernIpcNjclbytes,
            SysctlKey::KernIpcNmbclusters,
            SysctlKey::KernIpcSbmb_cnt,
            SysctlKey::KernIpcSbmb_cnt_floor,
            SysctlKey::KernIpcSbmb_cnt_peak,
            SysctlKey::KernIpcSbmb_limreached,
            SysctlKey::KernIpcSockbuf_waste_factor,
            SysctlKey::KernIpcSocket_debug,
            SysctlKey::KernIpcSodefunct_calls,
            SysctlKey::KernIpcSodefunctlog,
            SysctlKey::KernIpcSomaxconn,
            SysctlKey::KernIpcSoqlencomp,
            SysctlKey::KernIpcSoqlimitcompat,
            SysctlKey::KernIpcSorecvmincopy,
            SysctlKey::KernIpcSoreserveheadroom,
            SysctlKey::KernIpcSorestrictrecv,
            SysctlKey::KernIpcSorestrictsend,
            SysctlKey::KernIpcSosendbigcl_ignore_capab,
            SysctlKey::KernIpcSosendjcl,
            SysctlKey::KernIpcSosendjcl_ignore_capab,
            SysctlKey::KernIpcSosendminchain,
            SysctlKey::KernIpcSotcdb,
            SysctlKey::KernIpcSothrottlelog,
            SysctlKey::KernIpcThrottle_best_effort,
            SysctlKey::KernIpc_portbt,
            SysctlKey::KernIpc_voucher_trace_contents,
            SysctlKey::KernJetsam_aging_policy,
            SysctlKey::KernJob_control,
            SysctlKey::KernKdbgDebug,
            SysctlKey::KernKdbgExperimental_continuous,
            SysctlKey::KernKdbgOldest_time,
            SysctlKey::KernKern_feature_overrides,
            SysctlKey::KernKernelcacheuuid,
            SysctlKey::KernMaxfiles,
            SysctlKey::KernMaxfilesperproc,
            SysctlKey::KernMaxnbuf,
            SysctlKey::KernMaxproc,
            SysctlKey::KernMaxprocperuid,
            SysctlKey::KernMaxvnodes,
            SysctlKey::KernMemorystatus_apps_idle_delay_time,
            SysctlKey::KernMemorystatus_purge_on_critical,
            SysctlKey::KernMemorystatus_purge_on_urgent,
            SysctlKey::KernMemorystatus_purge_on_warning,
            SysctlKey::KernMemorystatus_sysprocs_idle_delay_time,
            SysctlKey::KernMinimalboot,
            SysctlKey::KernMonotonicPmis,
            SysctlKey::KernMonotonicRetrograde_updates,
            SysctlKey::KernMonotonicSupported,
            SysctlKey::KernMonotonicTask_thread_counting,
            SysctlKey::KernMsgbuf,
            SysctlKey::KernNamecache_disabled,
            SysctlKey::KernNbuf,
            SysctlKey::KernNetboot,
            SysctlKey::KernNgroups,
            SysctlKey::KernNisdomainname,
            SysctlKey::KernNum_files,
            SysctlKey::KernNum_recycledvnodes,
            SysctlKey::KernNum_tasks,
            SysctlKey::KernNum_taskthreads,
            SysctlKey::KernNum_threads,
            SysctlKey::KernNum_vnodes,
            SysctlKey::KernOsproductversion,
            SysctlKey::KernOsrelease,
            SysctlKey::KernOsrevision,
            SysctlKey::KernOstype,
            SysctlKey::KernOsversion,
            SysctlKey::KernPmtimeout,
            SysctlKey::KernPosix1version,
            SysctlKey::KernPosixSemMax,
            SysctlKey::KernPreheat_max_bytes,
            SysctlKey::KernPreheat_min_bytes,
            SysctlKey::KernProcname,
            SysctlKey::KernProgressmeter,
            SysctlKey::KernProgressmeterenable,
            SysctlKey::KernPthread_mutex_default_policy,
            SysctlKey::KernRage_vnode,
            SysctlKey::KernSafeboot,
            SysctlKey::KernSaved_ids,
            SysctlKey::KernSched,
            SysctlKey::KernSched_allow_NO_SMT_threads,
            SysctlKey::KernSched_enable_smt,
            SysctlKey::KernSecure_kernel,
            SysctlKey::KernSecurelevel,
            SysctlKey::KernShreg_private,
            SysctlKey::KernSingleuser,
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmFrag_count,
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmFrag_limit,
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmQueue_count,
            SysctlKey::KernSkywalkFlowswitchAwdl0IpfmQueue_limit,
            SysctlKey::KernSkywalkFlowswitchEn0IpfmFrag_count,
            SysctlKey::KernSkywalkFlowswitchEn0IpfmFrag_limit,
            SysctlKey::KernSkywalkFlowswitchEn0IpfmQueue_count,
            SysctlKey::KernSkywalkFlowswitchEn0IpfmQueue_limit,
            SysctlKey::KernSkywalkFlowswitchEn1IpfmFrag_count,
            SysctlKey::KernSkywalkFlowswitchEn1IpfmFrag_limit,
            SysctlKey::KernSkywalkFlowswitchEn1IpfmQueue_count,
            SysctlKey::KernSkywalkFlowswitchEn1IpfmQueue_limit,
            SysctlKey::KernSkywalkFlowswitchEn2IpfmFrag_count,
            SysctlKey::KernSkywalkFlowswitchEn2IpfmFrag_limit,
            SysctlKey::KernSkywalkFlowswitchEn2IpfmQueue_count,
            SysctlKey::KernSkywalkFlowswitchEn2IpfmQueue_limit,
            SysctlKey::KernSkywalkFlowswitchFlow_route_expire,
            SysctlKey::KernSkywalkFlowswitchIp_reass,
            SysctlKey::KernSkywalkFlowswitchIpfm_frag_ttl,
            SysctlKey::KernSkywalkFlowswitchIpfm_timeout_tcall_ival,
            SysctlKey::KernSkywalkNetifDefault_drop,
            SysctlKey::KernSkywalkRing_stat_enable,
            SysctlKey::KernSleep_abs_time,
            SysctlKey::KernSleeptime,
            SysctlKey::KernSlide,
            SysctlKey::KernSpeculative_prefetch_max,
            SysctlKey::KernSpeculative_prefetch_max_iosize,
            SysctlKey::KernSpeculative_reads_disabled,
            SysctlKey::KernStack_depth_max,
            SysctlKey::KernStack_size,
            SysctlKey::KernSugid_coredump,
            SysctlKey::KernSugid_scripts,
            SysctlKey::KernSysvSemmni,
            SysctlKey::KernSysvSemmns,
            SysctlKey::KernSysvSemmnu,
            SysctlKey::KernSysvSemmsl,
            SysctlKey::KernSysvSemume,
            SysctlKey::KernSysvShmall,
            SysctlKey::KernSysvShmmax,
            SysctlKey::KernSysvShmmin,
            SysctlKey::KernSysvShmmni,
            SysctlKey::KernSysvShmseg,
            SysctlKey::KernTask_exc_guard_default,
            SysctlKey::KernTfpPolicy,
            SysctlKey::KernThread_groups_supported,
            SysctlKey::KernThreadname,
            SysctlKey::KernTimerCoalescing_enabled,
            SysctlKey::KernTimerDeadline_tracking_bin_1,
            SysctlKey::KernTimerDeadline_tracking_bin_2,
            SysctlKey::KernTimerLongtermQlen,
            SysctlKey::KernTimerLongtermScan_interval,
            SysctlKey::KernTimerLongtermScan_limit,
            SysctlKey::KernTimerLongtermScan_pauses,
            SysctlKey::KernTimerLongtermThreshold,
            SysctlKey::KernTimer_coalesce_bg_ns_max,
            SysctlKey::KernTimer_coalesce_bg_scale,
            SysctlKey::KernTimer_coalesce_fp_ns_max,
            SysctlKey::KernTimer_coalesce_fp_scale,
            SysctlKey::KernTimer_coalesce_idle_entry_hard_deadline_max,
            SysctlKey::KernTimer_coalesce_kt_ns_max,
            SysctlKey::KernTimer_coalesce_kt_scale,
            SysctlKey::KernTimer_coalesce_tier0_ns_max,
            SysctlKey::KernTimer_coalesce_tier0_scale,
            SysctlKey::KernTimer_coalesce_tier1_ns_max,
            SysctlKey::KernTimer_coalesce_tier1_scale,
            SysctlKey::KernTimer_coalesce_tier2_ns_max,
            SysctlKey::KernTimer_coalesce_tier2_scale,
            SysctlKey::KernTimer_coalesce_tier3_ns_max,
            SysctlKey::KernTimer_coalesce_tier3_scale,
            SysctlKey::KernTimer_coalesce_tier4_ns_max,
            SysctlKey::KernTimer_coalesce_tier4_scale,
            SysctlKey::KernTimer_coalesce_tier5_ns_max,
            SysctlKey::KernTimer_coalesce_tier5_scale,
            SysctlKey::KernTimer_coalesce_ts_ns_max,
            SysctlKey::KernTimer_coalesce_ts_scale,
            SysctlKey::KernTimer_resort_threshold_ns,
            SysctlKey::KernTtyPtmx_max,
            SysctlKey::KernUlock_adaptive_spin_usecs,
            SysctlKey::KernUseractive_abs_time,
            SysctlKey::KernUserinactive_abs_time,
            SysctlKey::KernUsrstack,
            SysctlKey::KernUsrstack64,
            SysctlKey::KernUuid,
            SysctlKey::KernVersion,
            SysctlKey::KernVfsnspace,
            SysctlKey::KernVm_max_batch,
            SysctlKey::KernVm_max_delayed_work_limit,
            SysctlKey::KernVm_page_free_min,
            SysctlKey::KernVm_page_free_reserved,
            SysctlKey::KernVm_page_free_target,
            SysctlKey::KernVm_page_speculative_percentage,
            SysctlKey::KernVm_page_speculative_q_age_ms,
            SysctlKey::KernWake_abs_time,
            SysctlKey::KernWakereason,
            SysctlKey::KernWaketime,
            SysctlKey::KernWillshutdown,
            SysctlKey::KernWq_max_constrained_threads,
            SysctlKey::KernWq_max_threads,
            SysctlKey::KernWq_max_timer_interval_usecs,
            SysctlKey::KernWq_reduce_pool_window_usecs,
            SysctlKey::KernWq_stalled_window_usecs,
            SysctlKey::KernZleakActive,
            SysctlKey::KernZleakGlobal_threshold,
            SysctlKey::KernZleakMax_zonemap_size,
            SysctlKey::KernZleakZone_threshold,
            SysctlKey::KperfDebug_level,
            SysctlKey::KperfLimitsTimer_min_bg_period_ns,
            SysctlKey::KperfLimitsTimer_min_bg_pet_period_ns,
            SysctlKey::KperfLimitsTimer_min_period_ns,
            SysctlKey::KperfLimitsTimer_min_pet_period_ns,
            SysctlKey::KtraceBackground_pid,
            SysctlKey::KtraceConfigured_by,
            SysctlKey::KtraceOwning_pid,
            SysctlKey::KtraceState,
            SysctlKey::MachdepCpuAddress_bitsPhysical,
            SysctlKey::MachdepCpuAddress_bitsVirtual,
            SysctlKey::MachdepCpuArch_perfEvents,
            SysctlKey::MachdepCpuArch_perfEvents_number,
            SysctlKey::MachdepCpuArch_perfFixed_number,
            SysctlKey::MachdepCpuArch_perfFixed_width,
            SysctlKey::MachdepCpuArch_perfNumber,
            SysctlKey::MachdepCpuArch_perfVersion,
            SysctlKey::MachdepCpuArch_perfWidth,
            SysctlKey::MachdepCpuBrand,
            SysctlKey::MachdepCpuBrand_string,
            SysctlKey::MachdepCpuCacheL2_associativity,
            SysctlKey::MachdepCpuCacheLinesize,
            SysctlKey::MachdepCpuCacheSize,
            SysctlKey::MachdepCpuCore_count,
            SysctlKey::MachdepCpuCores_per_package,
            SysctlKey::MachdepCpuExtfamily,
            SysctlKey::MachdepCpuExtfeature_bits,
            SysctlKey::MachdepCpuExtfeatures,
            SysctlKey::MachdepCpuExtmodel,
            SysctlKey::MachdepCpuFamily,
            SysctlKey::MachdepCpuFeature_bits,
            SysctlKey::MachdepCpuFeatures,
            SysctlKey::MachdepCpuLeaf7_feature_bits,
            SysctlKey::MachdepCpuLeaf7_feature_bits_edx,
            SysctlKey::MachdepCpuLeaf7_features,
            SysctlKey::MachdepCpuLogical_per_package,
            SysctlKey::MachdepCpuMax_basic,
            SysctlKey::MachdepCpuMax_ext,
            SysctlKey::MachdepCpuMicrocode_version,
            SysctlKey::MachdepCpuModel,
            SysctlKey::MachdepCpuMwaitExtensions,
            SysctlKey::MachdepCpuMwaitLinesize_max,
            SysctlKey::MachdepCpuMwaitLinesize_min,
            SysctlKey::MachdepCpuMwaitSub_Cstates,
            SysctlKey::MachdepCpuProcessor_flag,
            SysctlKey::MachdepCpuSignature,
            SysctlKey::MachdepCpuStepping,
            SysctlKey::MachdepCpuThermalACNT_MCNT,
            SysctlKey::MachdepCpuThermalCore_power_limits,
            SysctlKey::MachdepCpuThermalDynamic_acceleration,
            SysctlKey::MachdepCpuThermalEnergy_policy,
            SysctlKey::MachdepCpuThermalFine_grain_clock_mod,
            SysctlKey::MachdepCpuThermalHardware_feedback,
            SysctlKey::MachdepCpuThermalInvariant_APIC_timer,
            SysctlKey::MachdepCpuThermalPackage_thermal_intr,
            SysctlKey::MachdepCpuThermalSensor,
            SysctlKey::MachdepCpuThermalThresholds,
            SysctlKey::MachdepCpuThread_count,
            SysctlKey::MachdepCpuTlbDataSmall,
            SysctlKey::MachdepCpuTlbDataSmall_level1,
            SysctlKey::MachdepCpuTlbInstLarge,
            SysctlKey::MachdepCpuTsc_cccDenominator,
            SysctlKey::MachdepCpuTsc_cccNumerator,
            SysctlKey::MachdepCpuVendor,
            SysctlKey::MachdepCpuXsaveExtended_state,
            SysctlKey::MachdepCpuXsaveExtended_state1,
            SysctlKey::MachdepEager_timer_evaluation_max,
            SysctlKey::MachdepEager_timer_evaluations,
            SysctlKey::MachdepMemmapACPINVS,
            SysctlKey::MachdepMemmapACPIReclaim,
            SysctlKey::MachdepMemmapConventional,
            SysctlKey::MachdepMemmapOther,
            SysctlKey::MachdepMemmapPalCode,
            SysctlKey::MachdepMemmapReserved,
            SysctlKey::MachdepMemmapRuntimeServices,
            SysctlKey::MachdepMemmapUnusable,
            SysctlKey::MachdepMiscFast_uexc_support,
            SysctlKey::MachdepMiscInterrupt_latency_max,
            SysctlKey::MachdepMiscNmis,
            SysctlKey::MachdepMiscPanic_restart_timeout,
            SysctlKey::MachdepMiscTimer_queue_trace,
            SysctlKey::MachdepPmapHashcnts,
            SysctlKey::MachdepPmapHashmax,
            SysctlKey::MachdepPmapHashwalks,
            SysctlKey::MachdepPmapKern_pv_reserve,
            SysctlKey::MachdepPmapKernel_text_ps,
            SysctlKey::MachdepTscAt_boot,
            SysctlKey::MachdepTscDeep_idle_rebase,
            SysctlKey::MachdepTscFrequency,
            SysctlKey::MachdepTscNanotimeGeneration,
            SysctlKey::MachdepTscNanotimeNs_base,
            SysctlKey::MachdepTscNanotimeScale,
            SysctlKey::MachdepTscNanotimeShift,
            SysctlKey::MachdepTscNanotimeTsc_base,
            SysctlKey::MachdepTscRebase_abs_time,
            SysctlKey::MachdepUser_idle_level,
            SysctlKey::MachdepVectorsIPI,
            SysctlKey::MachdepVectorsTimer,
            SysctlKey::MachdepX86_fp_simd_isr_uses,
            SysctlKey::MachdepXcpmBootplim,
            SysctlKey::MachdepXcpmBootpst,
            SysctlKey::MachdepXcpmCpu_thermal_level,
            SysctlKey::MachdepXcpmDeep_idle_count,
            SysctlKey::MachdepXcpmDeep_idle_last_stats,
            SysctlKey::MachdepXcpmDeep_idle_log,
            SysctlKey::MachdepXcpmDeep_idle_total_stats,
            SysctlKey::MachdepXcpmEpp_override,
            SysctlKey::MachdepXcpmForced_idle_period,
            SysctlKey::MachdepXcpmForced_idle_ratio,
            SysctlKey::MachdepXcpmGpu_thermal_level,
            SysctlKey::MachdepXcpmHard_plimit_max_100mhz_ratio,
            SysctlKey::MachdepXcpmHard_plimit_min_100mhz_ratio,
            SysctlKey::MachdepXcpmIo_control_disengages,
            SysctlKey::MachdepXcpmIo_control_engages,
            SysctlKey::MachdepXcpmIo_cst_control_enabled,
            SysctlKey::MachdepXcpmIo_epp_boost_enabled,
            SysctlKey::MachdepXcpmIo_filtered_reads,
            SysctlKey::MachdepXcpmIo_thermal_level,
            SysctlKey::MachdepXcpmMaxbusdelay,
            SysctlKey::MachdepXcpmMaxintdelay,
            SysctlKey::MachdepXcpmMbd_applications,
            SysctlKey::MachdepXcpmMbd_mode,
            SysctlKey::MachdepXcpmMbd_relaxations,
            SysctlKey::MachdepXcpmMid_applications,
            SysctlKey::MachdepXcpmMid_cst_control_limit,
            SysctlKey::MachdepXcpmMid_mode,
            SysctlKey::MachdepXcpmMid_mode_active,
            SysctlKey::MachdepXcpmMid_relaxations,
            SysctlKey::MachdepXcpmMode,
            SysctlKey::MachdepXcpmPcps_mode,
            SysctlKey::MachdepXcpmPcps_rt_override_mode,
            SysctlKey::MachdepXcpmPcps_rt_override_ns,
            SysctlKey::MachdepXcpmPerf_hints,
            SysctlKey::MachdepXcpmPower_source,
            SysctlKey::MachdepXcpmQos_txfr,
            SysctlKey::MachdepXcpmRatio_change_ratelimit_ns,
            SysctlKey::MachdepXcpmRatio_changes_total,
            SysctlKey::MachdepXcpmRing_boost_enabled,
            SysctlKey::MachdepXcpmSoft_plimit_max_100mhz_ratio,
            SysctlKey::MachdepXcpmSoft_plimit_min_100mhz_ratio,
            SysctlKey::MachdepXcpmTuib_enabled,
            SysctlKey::MachdepXcpmTuib_ns,
            SysctlKey::MachdepXcpmTuib_plimit_max_100mhz_ratio,
            SysctlKey::MachdepXcpmTuib_plimit_min_100mhz_ratio,
            SysctlKey::MachdepXcpmVectors_loaded_count,
            SysctlKey::NetAlfDefaultaction,
            SysctlKey::NetAlfLoglevel,
            SysctlKey::NetAlfMqcount,
            SysctlKey::NetAlfPerm,
            SysctlKey::NetCfilActive_count,
            SysctlKey::NetCfilClose_wait_timeout,
            SysctlKey::NetCfilDebug,
            SysctlKey::NetCfilLog,
            SysctlKey::NetCfilSbtrim,
            SysctlKey::NetCfilSock_attached_count,
            SysctlKey::NetClassqSfbAllocation,
            SysctlKey::NetClassqSfbDecrement,
            SysctlKey::NetClassqSfbHinterval,
            SysctlKey::NetClassqSfbHoldtime,
            SysctlKey::NetClassqSfbIncrement,
            SysctlKey::NetClassqSfbPboxtime,
            SysctlKey::NetClassqSfbRatelimit,
            SysctlKey::NetClassqTarget_qdelay,
            SysctlKey::NetClassqUpdate_interval,
            SysctlKey::NetClassqVerbose,
            SysctlKey::NetInet6Icmp6Errppslimit,
            SysctlKey::NetInet6Icmp6Nd6_accept_6to4,
            SysctlKey::NetInet6Icmp6Nd6_debug,
            SysctlKey::NetInet6Icmp6Nd6_delay,
            SysctlKey::NetInet6Icmp6Nd6_llreach_base,
            SysctlKey::NetInet6Icmp6Nd6_maxproxiedsol,
            SysctlKey::NetInet6Icmp6Nd6_maxsolstgt,
            SysctlKey::NetInet6Icmp6Nd6_mmaxtries,
            SysctlKey::NetInet6Icmp6Nd6_onlink_ns_rfc4861,
            SysctlKey::NetInet6Icmp6Nd6_optimistic_dad,
            SysctlKey::NetInet6Icmp6Nd6_prune,
            SysctlKey::NetInet6Icmp6Nd6_prune_lazy,
            SysctlKey::NetInet6Icmp6Nd6_umaxtries,
            SysctlKey::NetInet6Icmp6Nd6_useloopback,
            SysctlKey::NetInet6Icmp6Nodeinfo,
            SysctlKey::NetInet6Icmp6Prproxy_cnt,
            SysctlKey::NetInet6Icmp6Rappslimit,
            SysctlKey::NetInet6Icmp6Rediraccept,
            SysctlKey::NetInet6Icmp6Redirtimeout,
            SysctlKey::NetInet6Ip6Accept_rtadv,
            SysctlKey::NetInet6Ip6Adj_clear_hwcksum,
            SysctlKey::NetInet6Ip6Adj_partial_sum,
            SysctlKey::NetInet6Ip6Auto_flowlabel,
            SysctlKey::NetInet6Ip6Auto_linklocal,
            SysctlKey::NetInet6Ip6Check_interface,
            SysctlKey::NetInet6Ip6Checkinterface_debug,
            SysctlKey::NetInet6Ip6Clat_debug,
            SysctlKey::NetInet6Ip6Dad_count,
            SysctlKey::NetInet6Ip6Dad_enhanced,
            SysctlKey::NetInet6Ip6Defmcasthlim,
            SysctlKey::NetInet6Ip6Forwarding,
            SysctlKey::NetInet6Ip6Fragpackets,
            SysctlKey::NetInet6Ip6Gifhlim,
            SysctlKey::NetInet6Ip6Hdrnestlimit,
            SysctlKey::NetInet6Ip6Hlim,
            SysctlKey::NetInet6Ip6Input_perf,
            SysctlKey::NetInet6Ip6Input_perf_bins,
            SysctlKey::NetInet6Ip6Kame_version,
            SysctlKey::NetInet6Ip6Keepfaith,
            SysctlKey::NetInet6Ip6Log_interval,
            SysctlKey::NetInet6Ip6Maxchainsent,
            SysctlKey::NetInet6Ip6Maxdynroutes,
            SysctlKey::NetInet6Ip6Maxfragpackets,
            SysctlKey::NetInet6Ip6Maxfrags,
            SysctlKey::NetInet6Ip6Maxifdefrouters,
            SysctlKey::NetInet6Ip6Maxifprefixes,
            SysctlKey::NetInet6Ip6McastLoop,
            SysctlKey::NetInet6Ip6McastMaxgrpsrc,
            SysctlKey::NetInet6Ip6McastMaxsocksrc,
            SysctlKey::NetInet6Ip6Mcast_pmtu,
            SysctlKey::NetInet6Ip6Neighborgcthresh,
            SysctlKey::NetInet6Ip6Only_allow_rfc4193_prefixes,
            SysctlKey::NetInet6Ip6Output_perf,
            SysctlKey::NetInet6Ip6Output_perf_bins,
            SysctlKey::NetInet6Ip6Prefer_tempaddr,
            SysctlKey::NetInet6Ip6Redirect,
            SysctlKey::NetInet6Ip6Rr_prune,
            SysctlKey::NetInet6Ip6Rtexpire,
            SysctlKey::NetInet6Ip6Rtmaxcache,
            SysctlKey::NetInet6Ip6Rtminexpire,
            SysctlKey::NetInet6Ip6Select_src_expensive_secondary_if,
            SysctlKey::NetInet6Ip6Select_src_strong_end,
            SysctlKey::NetInet6Ip6Select_srcaddr_debug,
            SysctlKey::NetInet6Ip6Select_srcif_debug,
            SysctlKey::NetInet6Ip6Temppltime,
            SysctlKey::NetInet6Ip6Tempvltime,
            SysctlKey::NetInet6Ip6Use_defaultzone,
            SysctlKey::NetInet6Ip6Use_deprecated,
            SysctlKey::NetInet6Ip6Use_tempaddr,
            SysctlKey::NetInet6Ip6V6only,
            SysctlKey::NetInet6Ipsec6Ah_net_deflev,
            SysctlKey::NetInet6Ipsec6Ah_trans_deflev,
            SysctlKey::NetInet6Ipsec6Debug,
            SysctlKey::NetInet6Ipsec6Def_policy,
            SysctlKey::NetInet6Ipsec6Ecn,
            SysctlKey::NetInet6Ipsec6Esp_net_deflev,
            SysctlKey::NetInet6Ipsec6Esp_randpad,
            SysctlKey::NetInet6Ipsec6Esp_trans_deflev,
            SysctlKey::NetInet6MldDebug,
            SysctlKey::NetInet6MldGsrdelay,
            SysctlKey::NetInet6MldUse_allow,
            SysctlKey::NetInet6MldV1enable,
            SysctlKey::NetInet6MldV2enable,
            SysctlKey::NetInet6SendOpmode,
            SysctlKey::NetInet6SendOpstate,
            SysctlKey::NetInetIcmpBmcastecho,
            SysctlKey::NetInetIcmpDrop_redirect,
            SysctlKey::NetInetIcmpIcmplim,
            SysctlKey::NetInetIcmpLog_redirect,
            SysctlKey::NetInetIcmpMaskrepl,
            SysctlKey::NetInetIcmpTimestamp,
            SysctlKey::NetInetIgmpDebug,
            SysctlKey::NetInetIgmpDefault_version,
            SysctlKey::NetInetIgmpGsrdelay,
            SysctlKey::NetInetIgmpLegacysupp,
            SysctlKey::NetInetIgmpRecvifkludge,
            SysctlKey::NetInetIgmpSendlocal,
            SysctlKey::NetInetIgmpSendra,
            SysctlKey::NetInetIgmpV1enable,
            SysctlKey::NetInetIgmpV2enable,
            SysctlKey::NetInetIpAccept_sourceroute,
            SysctlKey::NetInetIpAdj_clear_hwcksum,
            SysctlKey::NetInetIpAdj_partial_sum,
            SysctlKey::NetInetIpCheck_interface,
            SysctlKey::NetInetIpCheckinterface_debug,
            SysctlKey::NetInetIpDummynetCurr_time,
            SysctlKey::NetInetIpDummynetDebug,
            SysctlKey::NetInetIpDummynetExpire,
            SysctlKey::NetInetIpDummynetExtract_heap,
            SysctlKey::NetInetIpDummynetHash_size,
            SysctlKey::NetInetIpDummynetMax_chain_len,
            SysctlKey::NetInetIpDummynetReady_heap,
            SysctlKey::NetInetIpDummynetRed_avg_pkt_size,
            SysctlKey::NetInetIpDummynetRed_lookup_depth,
            SysctlKey::NetInetIpDummynetRed_max_pkt_size,
            SysctlKey::NetInetIpDummynetSearch_steps,
            SysctlKey::NetInetIpDummynetSearches,
            SysctlKey::NetInetIpForwarding,
            SysctlKey::NetInetIpFragpackets,
            SysctlKey::NetInetIpGifttl,
            SysctlKey::NetInetIpLinklocalInAllowbadttl,
            SysctlKey::NetInetIpMaxchainsent,
            SysctlKey::NetInetIpMaxfragpackets,
            SysctlKey::NetInetIpMaxfragsperpacket,
            SysctlKey::NetInetIpMcastLoop,
            SysctlKey::NetInetIpMcastMaxgrpsrc,
            SysctlKey::NetInetIpMcastMaxsocksrc,
            SysctlKey::NetInetIpOutput_perf,
            SysctlKey::NetInetIpOutput_perf_bins,
            SysctlKey::NetInetIpPortrangeFirst,
            SysctlKey::NetInetIpPortrangeHifirst,
            SysctlKey::NetInetIpPortrangeHilast,
            SysctlKey::NetInetIpPortrangeLast,
            SysctlKey::NetInetIpPortrangeLowfirst,
            SysctlKey::NetInetIpPortrangeLowlast,
            SysctlKey::NetInetIpRandom_id,
            SysctlKey::NetInetIpRandom_id_collisions,
            SysctlKey::NetInetIpRandom_id_statistics,
            SysctlKey::NetInetIpRandom_id_total,
            SysctlKey::NetInetIpRedirect,
            SysctlKey::NetInetIpRfc6864,
            SysctlKey::NetInetIpRtexpire,
            SysctlKey::NetInetIpRtmaxcache,
            SysctlKey::NetInetIpRtminexpire,
            SysctlKey::NetInetIpRx_chaining,
            SysctlKey::NetInetIpRx_chainsz,
            SysctlKey::NetInetIpSelect_srcif_debug,
            SysctlKey::NetInetIpSendsourcequench,
            SysctlKey::NetInetIpSourceroute,
            SysctlKey::NetInetIpSubnets_are_local,
            SysctlKey::NetInetIpTtl,
            SysctlKey::NetInetIpsecAh_cleartos,
            SysctlKey::NetInetIpsecAh_net_deflev,
            SysctlKey::NetInetIpsecAh_offsetmask,
            SysctlKey::NetInetIpsecAh_trans_deflev,
            SysctlKey::NetInetIpsecBypass,
            SysctlKey::NetInetIpsecDebug,
            SysctlKey::NetInetIpsecDef_policy,
            SysctlKey::NetInetIpsecDfbit,
            SysctlKey::NetInetIpsecEcn,
            SysctlKey::NetInetIpsecEsp_net_deflev,
            SysctlKey::NetInetIpsecEsp_port,
            SysctlKey::NetInetIpsecEsp_randpad,
            SysctlKey::NetInetIpsecEsp_trans_deflev,
            SysctlKey::NetInetLog_restricted,
            SysctlKey::NetInetMptcpAllow_aggregate,
            SysctlKey::NetInetMptcpAlternate_port,
            SysctlKey::NetInetMptcpDbg_area,
            SysctlKey::NetInetMptcpDbg_level,
            SysctlKey::NetInetMptcpDss_csum,
            SysctlKey::NetInetMptcpEnable,
            SysctlKey::NetInetMptcpExpected_progress_headstart,
            SysctlKey::NetInetMptcpFail,
            SysctlKey::NetInetMptcpKeepalive,
            SysctlKey::NetInetMptcpMptcp_cap_retr,
            SysctlKey::NetInetMptcpNrto,
            SysctlKey::NetInetMptcpPcbcount,
            SysctlKey::NetInetMptcpProbecnt,
            SysctlKey::NetInetMptcpProbeto,
            SysctlKey::NetInetMptcpRto,
            SysctlKey::NetInetMptcpRto_thresh,
            SysctlKey::NetInetMptcpRtthist_thresh,
            SysctlKey::NetInetMptcpTw,
            SysctlKey::NetInetMptcpUserto,
            SysctlKey::NetInetRawMaxdgram,
            SysctlKey::NetInetRawPcbcount,
            SysctlKey::NetInetRawRecvspace,
            SysctlKey::NetInetTcpAcc_iaj_react_limit,
            SysctlKey::NetInetTcpAck_prioritize,
            SysctlKey::NetInetTcpAlways_keepalive,
            SysctlKey::NetInetTcpAutorcvbufmax,
            SysctlKey::NetInetTcpAutosndbufinc,
            SysctlKey::NetInetTcpAutosndbufmax,
            SysctlKey::NetInetTcpAutotunereorder,
            SysctlKey::NetInetTcpBackground_sockets,
            SysctlKey::NetInetTcpBackoff_maximum,
            SysctlKey::NetInetTcpBg_allowed_increase,
            SysctlKey::NetInetTcpBg_ss_fltsz,
            SysctlKey::NetInetTcpBg_target_qdelay,
            SysctlKey::NetInetTcpBg_tether_shift,
            SysctlKey::NetInetTcpBlackhole,
            SysctlKey::NetInetTcpBroken_peer_syn_rexmit_thres,
            SysctlKey::NetInetTcpCc_debug,
            SysctlKey::NetInetTcpChallengeack_limit,
            SysctlKey::NetInetTcpClear_tfocache,
            SysctlKey::NetInetTcpCubic_fast_convergence,
            SysctlKey::NetInetTcpCubic_sockets,
            SysctlKey::NetInetTcpCubic_tcp_friendliness,
            SysctlKey::NetInetTcpCubic_use_minrtt,
            SysctlKey::NetInetTcpDelayed_ack,
            SysctlKey::NetInetTcpDisable_access_to_stats,
            SysctlKey::NetInetTcpDisable_tcp_heuristics,
            SysctlKey::NetInetTcpDo_rfc5961,
            SysctlKey::NetInetTcpDo_tcpdrain,
            SysctlKey::NetInetTcpDoautorcvbuf,
            SysctlKey::NetInetTcpDoautosndbuf,
            SysctlKey::NetInetTcpDrop_synfin,
            SysctlKey::NetInetTcpEcn_initiate_out,
            SysctlKey::NetInetTcpEcn_negotiate_in,
            SysctlKey::NetInetTcpEcn_setup_percentage,
            SysctlKey::NetInetTcpEcn_timeout,
            SysctlKey::NetInetTcpEnable_tlp,
            SysctlKey::NetInetTcpFastopen,
            SysctlKey::NetInetTcpFastopen_backlog,
            SysctlKey::NetInetTcpIcmp_may_rst,
            SysctlKey::NetInetTcpInit_rtt_from_cache,
            SysctlKey::NetInetTcpKeepcnt,
            SysctlKey::NetInetTcpKeepidle,
            SysctlKey::NetInetTcpKeepinit,
            SysctlKey::NetInetTcpKeepintvl,
            SysctlKey::NetInetTcpLocal_slowstart_flightsize,
            SysctlKey::NetInetTcpLogEnable,
            SysctlKey::NetInetTcpLogEnable_usage,
            SysctlKey::NetInetTcpLogLevel_info,
            SysctlKey::NetInetTcpLogPrivacy,
            SysctlKey::NetInetTcpLogRate_current,
            SysctlKey::NetInetTcpLogRate_duration,
            SysctlKey::NetInetTcpLogRate_exceeded_total,
            SysctlKey::NetInetTcpLogRate_limit,
            SysctlKey::NetInetTcpLogRate_max,
            SysctlKey::NetInetTcpLogRtt_port,
            SysctlKey::NetInetTcpLogThflags_if_family,
            SysctlKey::NetInetTcpLog_in_vain,
            SysctlKey::NetInetTcpLro,
            SysctlKey::NetInetTcpLro_startcnt,
            SysctlKey::NetInetTcpLro_sz,
            SysctlKey::NetInetTcpLro_time,
            SysctlKey::NetInetTcpLrodbg,
            SysctlKey::NetInetTcpMax_persist_timeout,
            SysctlKey::NetInetTcpMaxseg_unacked,
            SysctlKey::NetInetTcpMicrouptime_init,
            SysctlKey::NetInetTcpMin_iaj_win,
            SysctlKey::NetInetTcpMinmss,
            SysctlKey::NetInetTcpMsl,
            SysctlKey::NetInetTcpMssdflt,
            SysctlKey::NetInetTcpNewreno_sockets,
            SysctlKey::NetInetTcpNow_init,
            SysctlKey::NetInetTcpPacketchain,
            SysctlKey::NetInetTcpPath_mtu_discovery,
            SysctlKey::NetInetTcpPcbcount,
            SysctlKey::NetInetTcpPmtud_blackhole_detection,
            SysctlKey::NetInetTcpPmtud_blackhole_mss,
            SysctlKey::NetInetTcpRandomize_ports,
            SysctlKey::NetInetTcpRcvsspktcnt,
            SysctlKey::NetInetTcpReassOverflows,
            SysctlKey::NetInetTcpRecv_allowed_iaj,
            SysctlKey::NetInetTcpRecv_throttle_minwin,
            SysctlKey::NetInetTcpRecvbg,
            SysctlKey::NetInetTcpRecvspace,
            SysctlKey::NetInetTcpRexmt_slop,
            SysctlKey::NetInetTcpRexmt_thresh,
            SysctlKey::NetInetTcpRfc1644,
            SysctlKey::NetInetTcpRfc3390,
            SysctlKey::NetInetTcpRfc3465,
            SysctlKey::NetInetTcpRfc3465_lim2,
            SysctlKey::NetInetTcpRtt_min,
            SysctlKey::NetInetTcpRtt_recvbg,
            SysctlKey::NetInetTcpSack,
            SysctlKey::NetInetTcpSack_globalholes,
            SysctlKey::NetInetTcpSack_globalmaxholes,
            SysctlKey::NetInetTcpSack_maxholes,
            SysctlKey::NetInetTcpSendspace,
            SysctlKey::NetInetTcpSlowlink_wsize,
            SysctlKey::NetInetTcpSlowstart_flightsize,
            SysctlKey::NetInetTcpSocket_unlocked_on_output,
            SysctlKey::NetInetTcpTcbhashsize,
            SysctlKey::NetInetTcpTcp_lq_overflow,
            SysctlKey::NetInetTcpTcp_resched_timerlist,
            SysctlKey::NetInetTcpTcp_timer_advanced,
            SysctlKey::NetInetTcpTimer_fastmode_idlemax,
            SysctlKey::NetInetTcpTso,
            SysctlKey::NetInetTcpTw_pcbcount,
            SysctlKey::NetInetTcpUse_newreno,
            SysctlKey::NetInetTcpV6mssdflt,
            SysctlKey::NetInetTcpWin_scale_factor,
            SysctlKey::NetInetUdpBlackhole,
            SysctlKey::NetInetUdpChecksum,
            SysctlKey::NetInetUdpLog_in_vain,
            SysctlKey::NetInetUdpMaxdgram,
            SysctlKey::NetInetUdpPcbcount,
            SysctlKey::NetInetUdpRandomize_ports,
            SysctlKey::NetInetUdpRecvspace,
            SysctlKey::NetIpsecDebug,
            SysctlKey::NetIpsecMax_pending_input,
            SysctlKey::NetIpsecRing_size,
            SysctlKey::NetIpsecRx_fsw_ring_size,
            SysctlKey::NetIpsecTx_fsw_ring_size,
            SysctlKey::NetIpsecVerify_interface_creation,
            SysctlKey::NetKeyAh_keymin,
            SysctlKey::NetKeyBlockacq_count,
            SysctlKey::NetKeyBlockacq_lifetime,
            SysctlKey::NetKeyDebug,
            SysctlKey::NetKeyEsp_auth,
            SysctlKey::NetKeyEsp_keymin,
            SysctlKey::NetKeyInt_random,
            SysctlKey::NetKeyLarval_lifetime,
            SysctlKey::NetKeyNatt_keepalive_interval,
            SysctlKey::NetKeyPrefered_oldsa,
            SysctlKey::NetKeySpi_maxval,
            SysctlKey::NetKeySpi_minval,
            SysctlKey::NetKeySpi_trycnt,
            SysctlKey::NetLinkBondDebug,
            SysctlKey::NetLinkBridgeDebug,
            SysctlKey::NetLinkBridgeInherit_mac,
            SysctlKey::NetLinkBridgeRtable_hash_size_max,
            SysctlKey::NetLinkBridgeRtable_prune_period,
            SysctlKey::NetLinkBridgeTxstart,
            SysctlKey::NetLinkEtherInetArp_llreach_base,
            SysctlKey::NetLinkEtherInetArp_unicast_lim,
            SysctlKey::NetLinkEtherInetHost_down_time,
            SysctlKey::NetLinkEtherInetKeep_announcements,
            SysctlKey::NetLinkEtherInetLog_arp_warnings,
            SysctlKey::NetLinkEtherInetMax_age,
            SysctlKey::NetLinkEtherInetMaxhold,
            SysctlKey::NetLinkEtherInetMaxhold_total,
            SysctlKey::NetLinkEtherInetMaxtries,
            SysctlKey::NetLinkEtherInetProbe_intvl,
            SysctlKey::NetLinkEtherInetProxyall,
            SysctlKey::NetLinkEtherInetPrune_intvl,
            SysctlKey::NetLinkEtherInetSend_conflicting_probes,
            SysctlKey::NetLinkEtherInetSendllconflict,
            SysctlKey::NetLinkEtherInetUseloopback,
            SysctlKey::NetLinkEtherInetVerbose,
            SysctlKey::NetLinkFakeBsd_mode,
            SysctlKey::NetLinkFakeBuflet_size,
            SysctlKey::NetLinkFakeCopypkt_mode,
            SysctlKey::NetLinkFakeDebug,
            SysctlKey::NetLinkFakeHwcsum,
            SysctlKey::NetLinkFakeIf_adv_intvl,
            SysctlKey::NetLinkFakeMax_mtu,
            SysctlKey::NetLinkFakeMultibuflet,
            SysctlKey::NetLinkFakeNxattach,
            SysctlKey::NetLinkFakeTx_drops,
            SysctlKey::NetLinkFakeTx_headroom,
            SysctlKey::NetLinkFakeTxstart,
            SysctlKey::NetLinkFakeUser_access,
            SysctlKey::NetLinkFakeWmm_mode,
            SysctlKey::NetLinkGenericSystemDelaybased_queue,
            SysctlKey::NetLinkGenericSystemDlil_input_sanity_check,
            SysctlKey::NetLinkGenericSystemDlil_input_threads,
            SysctlKey::NetLinkGenericSystemDlil_verbose,
            SysctlKey::NetLinkGenericSystemEnable_netagent,
            SysctlKey::NetLinkGenericSystemFlow_advisory,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_adjusted,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_bad_cksum,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_bad_rxoff,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_finalized_data,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_finalized_hdr,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_mode,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_forced,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_forced_bytes,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_rxoff_adj,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_partial_rxoff_forced,
            SysctlKey::NetLinkGenericSystemHwcksum_dbg_verified,
            SysctlKey::NetLinkGenericSystemHwcksum_in_invalidated,
            SysctlKey::NetLinkGenericSystemHwcksum_rx,
            SysctlKey::NetLinkGenericSystemHwcksum_tx,
            SysctlKey::NetLinkGenericSystemIf_verbose,
            SysctlKey::NetLinkGenericSystemIfcount,
            SysctlKey::NetLinkGenericSystemPort_usedEntry_count,
            SysctlKey::NetLinkGenericSystemPort_usedEntry_gen,
            SysctlKey::NetLinkGenericSystemPort_usedVerbose,
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_count,
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_last_if,
            SysctlKey::NetLinkGenericSystemPort_usedWakeuuid_not_set_last_time,
            SysctlKey::NetLinkGenericSystemRcvq_maxlen,
            SysctlKey::NetLinkGenericSystemRxpoll,
            SysctlKey::NetLinkGenericSystemRxpoll_decay,
            SysctlKey::NetLinkGenericSystemRxpoll_freeze_time,
            SysctlKey::NetLinkGenericSystemRxpoll_interval_pkts,
            SysctlKey::NetLinkGenericSystemRxpoll_interval_time,
            SysctlKey::NetLinkGenericSystemRxpoll_max,
            SysctlKey::NetLinkGenericSystemRxpoll_sample_time,
            SysctlKey::NetLinkGenericSystemRxpoll_wakeups_hiwat,
            SysctlKey::NetLinkGenericSystemRxpoll_wakeups_lowat,
            SysctlKey::NetLinkGenericSystemSndq_maxlen,
            SysctlKey::NetLinkGenericSystemStart_delay_disabled,
            SysctlKey::NetLinkGenericSystemStart_delayed,
            SysctlKey::NetLinkGenericSystemThreshold_interval,
            SysctlKey::NetLinkGenericSystemThreshold_notify,
            SysctlKey::NetLinkGenericSystemTx_chain_len_count,
            SysctlKey::NetLinkIptapLog,
            SysctlKey::NetLinkIptapTotal_tap_count,
            SysctlKey::NetLinkLoopbackDequeue_sc,
            SysctlKey::NetLinkLoopbackMax_dequeue,
            SysctlKey::NetLinkLoopbackSched_model,
            SysctlKey::NetLinkPktapCount_unknown_if_type,
            SysctlKey::NetLinkPktapLog,
            SysctlKey::NetLinkPktapTotal_tap_count,
            SysctlKey::NetLocalDgramMaxdgram,
            SysctlKey::NetLocalDgramRecvspace,
            SysctlKey::NetLocalInflight,
            SysctlKey::NetLocalStreamRecvspace,
            SysctlKey::NetLocalStreamSendspace,
            SysctlKey::NetLocalStreamTracemdns,
            SysctlKey::NetMpklogEnabled,
            SysctlKey::NetMpklogType,
            SysctlKey::NetMpklogVersion,
            SysctlKey::NetNdrv_multi_max_count,
            SysctlKey::NetNecpArena_count,
            SysctlKey::NetNecpClient_count,
            SysctlKey::NetNecpClient_fd_count,
            SysctlKey::NetNecpDebug,
            SysctlKey::NetNecpDrop_all_level,
            SysctlKey::NetNecpDrop_dest_debug,
            SysctlKey::NetNecpDrop_unentitled_level,
            SysctlKey::NetNecpIf_flow_count,
            SysctlKey::NetNecpIp_policy_count,
            SysctlKey::NetNecpNexus_flow_count,
            SysctlKey::NetNecpObserver_fd_count,
            SysctlKey::NetNecpObserver_message_limit,
            SysctlKey::NetNecpPass_interpose,
            SysctlKey::NetNecpPass_keepalives,
            SysctlKey::NetNecpPass_loopback,
            SysctlKey::NetNecpSession_count,
            SysctlKey::NetNecpSocket_flow_count,
            SysctlKey::NetNecpSocket_non_app_policy_count,
            SysctlKey::NetNecpSocket_policy_count,
            SysctlKey::NetNecpSysctl_arena_count,
            SysctlKey::NetNetagentActive_count,
            SysctlKey::NetNetagentDebug,
            SysctlKey::NetNetagentRegistered_count,
            SysctlKey::NetPktschedVerbose,
            SysctlKey::NetQosPolicyCapable_enabled,
            SysctlKey::NetQosPolicyRestrict_avapps,
            SysctlKey::NetQosPolicyRestricted,
            SysctlKey::NetQosPolicyWifi_enabled,
            SysctlKey::NetQosReset_dscp_to_wifi_ac_map,
            SysctlKey::NetQosVerbose,
            SysctlKey::NetRestricted_portEnforced,
            SysctlKey::NetRestricted_portVerbose,
            SysctlKey::NetSmbFsKern_deadtimer,
            SysctlKey::NetSmbFsKern_hard_deadtimer,
            SysctlKey::NetSmbFsKern_soft_deadtimer,
            SysctlKey::NetSmbFsLoglevel,
            SysctlKey::NetSmbFsMaxread,
            SysctlKey::NetSmbFsMaxsegreadsize,
            SysctlKey::NetSmbFsMaxsegwritesize,
            SysctlKey::NetSmbFsMaxwrite,
            SysctlKey::NetSmbFsTcprcvbuf,
            SysctlKey::NetSmbFsTcpsndbuf,
            SysctlKey::NetSmbFsVersion,
            SysctlKey::NetStatistics_privcheck,
            SysctlKey::NetStatsDebug,
            SysctlKey::NetStatsRecvspace,
            SysctlKey::NetStatsSendspace,
            SysctlKey::NetSystmKctlAutorcvbufhigh,
            SysctlKey::NetSystmKctlAutorcvbufmax,
            SysctlKey::NetSystmKctlDebug,
            SysctlKey::NetUtunMax_pending_input,
            SysctlKey::NetUtunRing_size,
            SysctlKey::NetUtunRx_fsw_ring_size,
            SysctlKey::NetUtunTx_fsw_ring_size,
            SysctlKey::SecurityMacAmfiForce_policy,
            SysctlKey::SecurityMacAmfiHsp_enable,
            SysctlKey::SecurityMacAmfiVerbose_logging,
            SysctlKey::SecurityMacAspActive_rule_version,
            SysctlKey::SecurityMacAspCache_allocation_count,
            SysctlKey::SecurityMacAspCache_entry_count,
            SysctlKey::SecurityMacAspCache_release_count,
            SysctlKey::SecurityMacAspExec_hook_count,
            SysctlKey::SecurityMacAspExec_hook_sleep_time,
            SysctlKey::SecurityMacAspExec_hook_work_time,
            SysctlKey::SecurityMacAspLibrary_hook_count,
            SysctlKey::SecurityMacAspLibrary_hook_time,
            SysctlKey::SecurityMacAspLibrary_sleep_time,
            SysctlKey::SecurityMacAspPending_evaluation_count,
            SysctlKey::SecurityMacDevice_enforce,
            SysctlKey::SecurityMacEndpointsecurityLog_level,
            SysctlKey::SecurityMacLabelvnodes,
            SysctlKey::SecurityMacMax_slots,
            SysctlKey::SecurityMacPipe_enforce,
            SysctlKey::SecurityMacPlatform_exec_logging,
            SysctlKey::SecurityMacPosixsem_enforce,
            SysctlKey::SecurityMacPosixshm_enforce,
            SysctlKey::SecurityMacProc_enforce,
            SysctlKey::SecurityMacQtnSandbox_enforce,
            SysctlKey::SecurityMacQtnUser_approved_exec,
            SysctlKey::SecurityMacSandboxAudio_active,
            SysctlKey::SecurityMacSandboxSentinel,
            SysctlKey::SecurityMacSocket_enforce,
            SysctlKey::SecurityMacSystem_enforce,
            SysctlKey::SecurityMacSysvmsg_enforce,
            SysctlKey::SecurityMacSysvsem_enforce,
            SysctlKey::SecurityMacSysvshm_enforce,
            SysctlKey::SecurityMacVm_enforce,
            SysctlKey::SecurityMacVnode_enforce,
            SysctlKey::SecurityMacVnode_label_count,
            SysctlKey::UserBc_base_max,
            SysctlKey::UserBc_dim_max,
            SysctlKey::UserBc_scale_max,
            SysctlKey::UserBc_string_max,
            SysctlKey::UserColl_weights_max,
            SysctlKey::UserCs_path,
            SysctlKey::UserExpr_nest_max,
            SysctlKey::UserLine_max,
            SysctlKey::UserPosix2_c_bind,
            SysctlKey::UserPosix2_c_dev,
            SysctlKey::UserPosix2_char_term,
            SysctlKey::UserPosix2_fort_dev,
            SysctlKey::UserPosix2_fort_run,
            SysctlKey::UserPosix2_localedef,
            SysctlKey::UserPosix2_sw_dev,
            SysctlKey::UserPosix2_upe,
            SysctlKey::UserPosix2_version,
            SysctlKey::UserRe_dup_max,
            SysctlKey::UserStream_max,
            SysctlKey::UserTzname_max,
            SysctlKey::VfsGenericAlways_do_fullfsync,
            SysctlKey::VfsGenericApfsAllocated,
            SysctlKey::VfsGenericApfsFusion_elevator_throttle,
            SysctlKey::VfsGenericApfsFusion_lc_rc_promotion_threshold_mult,
            SysctlKey::VfsGenericApfsFusion_paranoia_level,
            SysctlKey::VfsGenericApfsFusion_promoter_queue_limit,
            SysctlKey::VfsGenericApfsFusion_promoter_throttle,
            SysctlKey::VfsGenericApfsFusion_rc_flags,
            SysctlKey::VfsGenericApfsFusion_rc_promotion_size_limit_mb,
            SysctlKey::VfsGenericApfsFusion_rc_promotion_threshold_mult,
            SysctlKey::VfsGenericApfsFusion_slow_io_threshold,
            SysctlKey::VfsGenericApfsFusion_swapfile_backoff,
            SysctlKey::VfsGenericApfsFusion_verbosity_flags,
            SysctlKey::VfsGenericApfsFusion_w2rc_filled_ratio_threshold,
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_low,
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_med,
            SysctlKey::VfsGenericApfsFusion_wbc_backoff_wmk_reenable,
            SysctlKey::VfsGenericApfsFusion_wbc_buffersize,
            SysctlKey::VfsGenericApfsFusion_wbc_elevator_wmk,
            SysctlKey::VfsGenericAutofsVnode_recycle_on_inactive,
            SysctlKey::VfsGenericHfsAllocated,
            SysctlKey::VfsGenericHfsJnlKdebugTrim,
            SysctlKey::VfsGenericHfsJnlTrim_flush,
            SysctlKey::VfsGenericHfsKdebugAllocation,
            SysctlKey::VfsGenericMaxtypenum,
            SysctlKey::VfsGenericNfsClientAccess_cache_timeout,
            SysctlKey::VfsGenericNfsClientAccess_delete,
            SysctlKey::VfsGenericNfsClientAccess_dotzfs,
            SysctlKey::VfsGenericNfsClientAccess_for_getattr,
            SysctlKey::VfsGenericNfsClientAllow_async,
            SysctlKey::VfsGenericNfsClientCallback_port,
            SysctlKey::VfsGenericNfsClientDebug_ctl,
            SysctlKey::VfsGenericNfsClientDefault_nfs4domain,
            SysctlKey::VfsGenericNfsClientIdmap_ctrl,
            SysctlKey::VfsGenericNfsClientInitialdowndelay,
            SysctlKey::VfsGenericNfsClientIosize,
            SysctlKey::VfsGenericNfsClientIs_mobile,
            SysctlKey::VfsGenericNfsClientLockd_mounts,
            SysctlKey::VfsGenericNfsClientMax_async_writes,
            SysctlKey::VfsGenericNfsClientNextdowndelay,
            SysctlKey::VfsGenericNfsClientNfsiod_thread_count,
            SysctlKey::VfsGenericNfsClientNfsiod_thread_max,
            SysctlKey::VfsGenericNfsClientReadlink_nocache,
            SysctlKey::VfsGenericNfsClientRoot_steals_gss_context,
            SysctlKey::VfsGenericNfsClientSquishy_flags,
            SysctlKey::VfsGenericNfsClientStatfs_rate_limit,
            SysctlKey::VfsGenericNfsServerAsync,
            SysctlKey::VfsGenericNfsServerExport_hash_size,
            SysctlKey::VfsGenericNfsServerFsevents,
            SysctlKey::VfsGenericNfsServerGss_context_ttl,
            SysctlKey::VfsGenericNfsServerNfsd_sock_idle_timeout,
            SysctlKey::VfsGenericNfsServerNfsd_tcp_connections,
            SysctlKey::VfsGenericNfsServerNfsd_thread_count,
            SysctlKey::VfsGenericNfsServerNfsd_thread_max,
            SysctlKey::VfsGenericNfsServerReqcache_size,
            SysctlKey::VfsGenericNfsServerRequest_queue_length,
            SysctlKey::VfsGenericNfsServerRequire_resv_port,
            SysctlKey::VfsGenericNfsServerUpcall_queue_count,
            SysctlKey::VfsGenericNfsServerUpcall_queue_limit,
            SysctlKey::VfsGenericNfsServerUpcall_queue_max_seen,
            SysctlKey::VfsGenericNfsServerUse_upcall_svc,
            SysctlKey::VfsGenericNfsServerUser_stats,
            SysctlKey::VfsGenericNfsServerWg_delay,
            SysctlKey::VfsGenericNfsServerWg_delay_v3,
            SysctlKey::VfsGenericRoot_unmounted_cleanly,
            SysctlKey::VfsGenericSync_timeout,
            SysctlKey::VfsNspacePrevent_materialization,
            SysctlKey::VfsNspaceResolver,
            SysctlKey::VfsNspaceThread_prevent_materialization,
            SysctlKey::VfsNummntops,
            SysctlKey::VmAll_reusable_calls,
            SysctlKey::VmAll_reuse_calls,
            SysctlKey::VmCan_reuse_failure,
            SysctlKey::VmCan_reuse_success,
            SysctlKey::VmCompressor_available,
            SysctlKey::VmCompressor_bytes_used,
            SysctlKey::VmCompressor_compressed_bytes,
            SysctlKey::VmCompressor_eval_period_in_msecs,
            SysctlKey::VmCompressor_input_bytes,
            SysctlKey::VmCompressor_is_active,
            SysctlKey::VmCompressor_min_csegs_per_major_compaction,
            SysctlKey::VmCompressor_mode,
            SysctlKey::VmCompressor_sample_max_in_msecs,
            SysctlKey::VmCompressor_sample_min_in_msecs,
            SysctlKey::VmCompressor_swapout_target_age,
            SysctlKey::VmCompressor_thrashing_min_per_10msecs,
            SysctlKey::VmCompressor_thrashing_threshold_per_10msecs,
            SysctlKey::VmCompressor_timing_enabled,
            SysctlKey::VmCopied_on_read,
            SysctlKey::VmCorpse_footprint_count,
            SysctlKey::VmCorpse_footprint_full,
            SysctlKey::VmCorpse_footprint_no_buf,
            SysctlKey::VmCorpse_footprint_size_avg,
            SysctlKey::VmCorpse_footprint_size_max,
            SysctlKey::VmCs_all_vnodes,
            SysctlKey::VmCs_blob_count,
            SysctlKey::VmCs_blob_count_peak,
            SysctlKey::VmCs_blob_size,
            SysctlKey::VmCs_blob_size_max,
            SysctlKey::VmCs_blob_size_peak,
            SysctlKey::VmCs_debug,
            SysctlKey::VmCs_debug_fail_on_unsigned_code,
            SysctlKey::VmCs_debug_unsigned_exec_failures,
            SysctlKey::VmCs_debug_unsigned_mmap_failures,
            SysctlKey::VmCs_enforcement_panic,
            SysctlKey::VmCs_force_hard,
            SysctlKey::VmCs_force_kill,
            SysctlKey::VmCs_library_validation,
            SysctlKey::VmCs_process_enforcement,
            SysctlKey::VmCs_system_enforcement,
            SysctlKey::VmDarkwake_mode,
            SysctlKey::VmFree_shared,
            SysctlKey::VmGlobal_no_user_wire_amount,
            SysctlKey::VmGlobal_user_wire_limit,
            SysctlKey::VmIopl_pages_tainted,
            SysctlKey::VmKern_lpage_count,
            SysctlKey::VmLoadavg,
            SysctlKey::VmLz4_compressed_bytes,
            SysctlKey::VmLz4_compression_failures,
            SysctlKey::VmLz4_compressions,
            SysctlKey::VmLz4_decompressed_bytes,
            SysctlKey::VmLz4_decompressions,
            SysctlKey::VmLz4_max_failure_run_length,
            SysctlKey::VmLz4_max_failure_skips,
            SysctlKey::VmLz4_max_preselects,
            SysctlKey::VmLz4_profitable_bytes,
            SysctlKey::VmLz4_run_continue_bytes,
            SysctlKey::VmLz4_run_preselection_threshold,
            SysctlKey::VmLz4_threshold,
            SysctlKey::VmLz4_wk_compression_delta,
            SysctlKey::VmLz4_wk_compression_negative_delta,
            SysctlKey::VmMadvise_free_debug,
            SysctlKey::VmMemory_pressure,
            SysctlKey::VmPage_busy_absent_skipped,
            SysctlKey::VmPage_cleaned_count,
            SysctlKey::VmPage_free_count,
            SysctlKey::VmPage_free_wanted,
            SysctlKey::VmPage_pageable_external_count,
            SysctlKey::VmPage_pageable_internal_count,
            SysctlKey::VmPage_purgeable_count,
            SysctlKey::VmPage_purgeable_wired_count,
            SysctlKey::VmPage_reusable_count,
            SysctlKey::VmPage_speculative_count,
            SysctlKey::VmPageout_freed_cleaned,
            SysctlKey::VmPageout_freed_external,
            SysctlKey::VmPageout_freed_speculative,
            SysctlKey::VmPageout_inactive_clean,
            SysctlKey::VmPageout_inactive_dirty_external,
            SysctlKey::VmPageout_inactive_dirty_internal,
            SysctlKey::VmPageout_inactive_used,
            SysctlKey::VmPageout_speculative_clean,
            SysctlKey::VmPages,
            SysctlKey::VmPagesize,
            SysctlKey::VmPartial_reusable_calls,
            SysctlKey::VmPartial_reuse_calls,
            SysctlKey::VmPrefault_nb_bailout,
            SysctlKey::VmPrefault_nb_pages,
            SysctlKey::VmProtect_privileged_from_untrusted,
            SysctlKey::VmReusable_failure,
            SysctlKey::VmReusable_nonwritable,
            SysctlKey::VmReusable_pages_shared,
            SysctlKey::VmReusable_reclaimed,
            SysctlKey::VmReusable_shared,
            SysctlKey::VmReusable_success,
            SysctlKey::VmReuse_failure,
            SysctlKey::VmReuse_success,
            SysctlKey::VmShared_region_pager_copied,
            SysctlKey::VmShared_region_pager_reclaimed,
            SysctlKey::VmShared_region_pager_slid,
            SysctlKey::VmShared_region_pager_slid_error,
            SysctlKey::VmShared_region_persistence,
            SysctlKey::VmShared_region_trace_level,
            SysctlKey::VmShared_region_unnest_logging,
            SysctlKey::VmShared_region_version,
            SysctlKey::VmSwapfileprefix,
            SysctlKey::VmSwapusage,
            SysctlKey::VmUc_decompressions,
            SysctlKey::VmUpl_pages_tainted,
            SysctlKey::VmUser_wire_limit,
            SysctlKey::VmVm_clump_promote_threshold,
            SysctlKey::VmVm_copy_src_large,
            SysctlKey::VmVm_copy_src_not_internal,
            SysctlKey::VmVm_copy_src_not_symmetric,
            SysctlKey::VmVm_create_upl_extra_cow,
            SysctlKey::VmVm_create_upl_extra_cow_pages,
            SysctlKey::VmVm_create_upl_lookup_failure_copy,
            SysctlKey::VmVm_create_upl_lookup_failure_write,
            SysctlKey::VmVm_debug_events,
            SysctlKey::VmVm_do_collapse_compressor,
            SysctlKey::VmVm_do_collapse_compressor_pages,
            SysctlKey::VmVm_do_collapse_terminate,
            SysctlKey::VmVm_do_collapse_terminate_failure,
            SysctlKey::VmVm_max_kernel_address,
            SysctlKey::VmVm_min_kernel_address,
            SysctlKey::VmVm_page_background_count,
            SysctlKey::VmVm_page_background_exclude_external,
            SysctlKey::VmVm_page_background_external_count,
            SysctlKey::VmVm_page_background_internal_count,
            SysctlKey::VmVm_page_background_mode,
            SysctlKey::VmVm_page_background_promoted_count,
            SysctlKey::VmVm_page_background_target,
            SysctlKey::VmVm_page_external_count,
            SysctlKey::VmVm_page_filecache_min,
            SysctlKey::VmVm_page_free_target,
            SysctlKey::VmVm_page_xpmapped_min,
            SysctlKey::VmVm_pageout_considered_bq_external,
            SysctlKey::VmVm_pageout_considered_bq_internal,
            SysctlKey::VmVm_pageout_rejected_bq_external,
            SysctlKey::VmVm_pageout_rejected_bq_internal,
            SysctlKey::VmVm_ripe_target_age_in_secs,
            SysctlKey::VmVm_should_cow_but_wired,
            SysctlKey::VmWk_catime,
            SysctlKey::VmWk_compressed_bytes_exclusive,
            SysctlKey::VmWk_compressed_bytes_total,
            SysctlKey::VmWk_compression_failures,
            SysctlKey::VmWk_compressions,
            SysctlKey::VmWk_compressions_exclusive,
            SysctlKey::VmWk_datime,
            SysctlKey::VmWk_decompressed_bytes,
            SysctlKey::VmWk_decompressions,
            SysctlKey::VmWk_mzv_compressions,
            SysctlKey::VmWk_sv_compressions,
            SysctlKey::VmWk_sv_decompressions,
            SysctlKey::VmWkdm_reeval_threshold,
            SysctlKey::VmWkh_catime,
            SysctlKey::VmWkh_compressions,
            SysctlKey::VmWkh_datime,
            SysctlKey::VmWkh_decompressions,
            SysctlKey::VmWks_catime,
            SysctlKey::VmWks_compressed_bytes,
            SysctlKey::VmWks_compression_failures,
            SysctlKey::VmWks_compressions,
            SysctlKey::VmWks_datime,
            SysctlKey::VmWks_decompressions,
            SysctlKey::VmWks_sv_compressions,
        ]
    }
}
