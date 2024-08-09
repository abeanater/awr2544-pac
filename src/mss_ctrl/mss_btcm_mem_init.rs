#[doc = "Register `MSS_BTCM_MEM_INIT` reader"]
pub type R = crate::R<MssBtcmMemInitSpec>;
#[doc = "Register `MSS_BTCM_MEM_INIT` writer"]
pub type W = crate::W<MssBtcmMemInitSpec>;
#[doc = "Field `mem_init` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the B0/1TCM banks of CR5A/B"]
pub type MemInitR = crate::BitReader;
#[doc = "Field `mem_init` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the B0/1TCM banks of CR5A/B"]
pub type MemInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the B0/1TCM banks of CR5A/B"]
    #[inline(always)]
    pub fn mem_init(&self) -> MemInitR {
        MemInitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the B0/1TCM banks of CR5A/B"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init(&mut self) -> MemInitW<MssBtcmMemInitSpec> {
        MemInitW::new(self, 0)
    }
}
#[doc = "MSS_BTCM_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_btcm_mem_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_btcm_mem_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssBtcmMemInitSpec;
impl crate::RegisterSpec for MssBtcmMemInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_btcm_mem_init::R`](R) reader structure"]
impl crate::Readable for MssBtcmMemInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_btcm_mem_init::W`](W) writer structure"]
impl crate::Writable for MssBtcmMemInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_BTCM_MEM_INIT to value 0"]
impl crate::Resettable for MssBtcmMemInitSpec {
    const RESET_VALUE: u32 = 0;
}
