#[doc = "Register `MSS_GPADC_MEM_INIT_STATUS` reader"]
pub type R = crate::R<MssGpadcMemInitStatusSpec>;
#[doc = "Register `MSS_GPADC_MEM_INIT_STATUS` writer"]
pub type W = crate::W<MssGpadcMemInitStatusSpec>;
#[doc = "Field `mem0_status` reader - 0:0\\]
1'b0: No initialization is happening for MSS_GPADC_DATA_MEM 1'b1: Initialization is in progress for MSS_GPADC_DATA_MEM"]
pub type Mem0StatusR = crate::BitReader;
#[doc = "Field `mem0_status` writer - 0:0\\]
1'b0: No initialization is happening for MSS_GPADC_DATA_MEM 1'b1: Initialization is in progress for MSS_GPADC_DATA_MEM"]
pub type Mem0StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for MSS_GPADC_DATA_MEM 1'b1: Initialization is in progress for MSS_GPADC_DATA_MEM"]
    #[inline(always)]
    pub fn mem0_status(&self) -> Mem0StatusR {
        Mem0StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for MSS_GPADC_DATA_MEM 1'b1: Initialization is in progress for MSS_GPADC_DATA_MEM"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_status(&mut self) -> Mem0StatusW<MssGpadcMemInitStatusSpec> {
        Mem0StatusW::new(self, 0)
    }
}
#[doc = "MSS_GPADC_MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_mem_init_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_mem_init_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssGpadcMemInitStatusSpec;
impl crate::RegisterSpec for MssGpadcMemInitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_gpadc_mem_init_status::R`](R) reader structure"]
impl crate::Readable for MssGpadcMemInitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_gpadc_mem_init_status::W`](W) writer structure"]
impl crate::Writable for MssGpadcMemInitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_GPADC_MEM_INIT_STATUS to value 0"]
impl crate::Resettable for MssGpadcMemInitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
