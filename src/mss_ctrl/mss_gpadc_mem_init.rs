#[doc = "Register `MSS_GPADC_MEM_INIT` reader"]
pub type R = crate::R<MssGpadcMemInitSpec>;
#[doc = "Register `MSS_GPADC_MEM_INIT` writer"]
pub type W = crate::W<MssGpadcMemInitSpec>;
#[doc = "Field `mem0_init` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_GPADC_DATA_MEM. Value in each row is initialized to 0x00_0000_03FF"]
pub type Mem0InitR = crate::BitReader;
#[doc = "Field `mem0_init` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_GPADC_DATA_MEM. Value in each row is initialized to 0x00_0000_03FF"]
pub type Mem0InitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_GPADC_DATA_MEM. Value in each row is initialized to 0x00_0000_03FF"]
    #[inline(always)]
    pub fn mem0_init(&self) -> Mem0InitR {
        Mem0InitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the MSS_GPADC_DATA_MEM. Value in each row is initialized to 0x00_0000_03FF"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_init(&mut self) -> Mem0InitW<MssGpadcMemInitSpec> {
        Mem0InitW::new(self, 0)
    }
}
#[doc = "MSS_GPADC_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gpadc_mem_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gpadc_mem_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssGpadcMemInitSpec;
impl crate::RegisterSpec for MssGpadcMemInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_gpadc_mem_init::R`](R) reader structure"]
impl crate::Readable for MssGpadcMemInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_gpadc_mem_init::W`](W) writer structure"]
impl crate::Writable for MssGpadcMemInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_GPADC_MEM_INIT to value 0"]
impl crate::Resettable for MssGpadcMemInitSpec {
    const RESET_VALUE: u32 = 0;
}
