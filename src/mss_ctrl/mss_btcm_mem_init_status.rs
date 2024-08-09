#[doc = "Register `MSS_BTCM_MEM_INIT_STATUS` reader"]
pub type R = crate::R<MssBtcmMemInitStatusSpec>;
#[doc = "Register `MSS_BTCM_MEM_INIT_STATUS` writer"]
pub type W = crate::W<MssBtcmMemInitStatusSpec>;
#[doc = "Field `mem_status` reader - 0:0\\]
1'b0: No initialization is happening for B0/1TCM banks of CR5A/B 1'b1: Initialization is in progress for B0/1TCM banks of CR5A/B"]
pub type MemStatusR = crate::BitReader;
#[doc = "Field `mem_status` writer - 0:0\\]
1'b0: No initialization is happening for B0/1TCM banks of CR5A/B 1'b1: Initialization is in progress for B0/1TCM banks of CR5A/B"]
pub type MemStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for B0/1TCM banks of CR5A/B 1'b1: Initialization is in progress for B0/1TCM banks of CR5A/B"]
    #[inline(always)]
    pub fn mem_status(&self) -> MemStatusR {
        MemStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for B0/1TCM banks of CR5A/B 1'b1: Initialization is in progress for B0/1TCM banks of CR5A/B"]
    #[inline(always)]
    #[must_use]
    pub fn mem_status(&mut self) -> MemStatusW<MssBtcmMemInitStatusSpec> {
        MemStatusW::new(self, 0)
    }
}
#[doc = "MSS_BTCM_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_btcm_mem_init_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_btcm_mem_init_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBtcmMemInitStatusSpec;
impl crate::RegisterSpec for MssBtcmMemInitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_btcm_mem_init_status::R`](R) reader structure"]
impl crate::Readable for MssBtcmMemInitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_btcm_mem_init_status::W`](W) writer structure"]
impl crate::Writable for MssBtcmMemInitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BTCM_MEM_INIT_STATUS to value 0"]
impl crate::Resettable for MssBtcmMemInitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
