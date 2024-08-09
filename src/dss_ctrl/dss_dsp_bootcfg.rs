#[doc = "Register `DSS_DSP_BOOTCFG` reader"]
pub type R = crate::R<DssDspBootcfgSpec>;
#[doc = "Register `DSS_DSP_BOOTCFG` writer"]
pub type W = crate::W<DssDspBootcfgSpec>;
#[doc = "Field `ISTP_RST_VAL` reader - 21:0\\]
DSP Boot Configuration : Reset Vector"]
pub type IstpRstValR = crate::FieldReader<u32>;
#[doc = "Field `ISTP_RST_VAL` writer - 21:0\\]
DSP Boot Configuration : Reset Vector"]
pub type IstpRstValW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `L1D_CACHE_MODE` reader - 24:24\\]
DSP Boot Configuration : L1D Cache Mode"]
pub type L1dCacheModeR = crate::BitReader;
#[doc = "Field `L1D_CACHE_MODE` writer - 24:24\\]
DSP Boot Configuration : L1D Cache Mode"]
pub type L1dCacheModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1P_CACHE_MODE` reader - 25:25\\]
DSP Boot Configuration : L1P Cache Mode"]
pub type L1pCacheModeR = crate::BitReader;
#[doc = "Field `L1P_CACHE_MODE` writer - 25:25\\]
DSP Boot Configuration : L1P Cache Mode"]
pub type L1pCacheModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
DSP Boot Configuration : Reset Vector"]
    #[inline(always)]
    pub fn istp_rst_val(&self) -> IstpRstValR {
        IstpRstValR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
DSP Boot Configuration : L1D Cache Mode"]
    #[inline(always)]
    pub fn l1d_cache_mode(&self) -> L1dCacheModeR {
        L1dCacheModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
DSP Boot Configuration : L1P Cache Mode"]
    #[inline(always)]
    pub fn l1p_cache_mode(&self) -> L1pCacheModeR {
        L1pCacheModeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
DSP Boot Configuration : Reset Vector"]
    #[inline(always)]
    #[must_use]
    pub fn istp_rst_val(&mut self) -> IstpRstValW<DssDspBootcfgSpec> {
        IstpRstValW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DSP Boot Configuration : L1D Cache Mode"]
    #[inline(always)]
    #[must_use]
    pub fn l1d_cache_mode(&mut self) -> L1dCacheModeW<DssDspBootcfgSpec> {
        L1dCacheModeW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
DSP Boot Configuration : L1P Cache Mode"]
    #[inline(always)]
    #[must_use]
    pub fn l1p_cache_mode(&mut self) -> L1pCacheModeW<DssDspBootcfgSpec> {
        L1pCacheModeW::new(self, 25)
    }
}
#[doc = "DSS_DSP_BOOTCFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_dsp_bootcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_dsp_bootcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssDspBootcfgSpec;
impl crate::RegisterSpec for DssDspBootcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_dsp_bootcfg::R`](R) reader structure"]
impl crate::Readable for DssDspBootcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_dsp_bootcfg::W`](W) writer structure"]
impl crate::Writable for DssDspBootcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_DSP_BOOTCFG to value 0"]
impl crate::Resettable for DssDspBootcfgSpec {
    const RESET_VALUE: u32 = 0;
}
