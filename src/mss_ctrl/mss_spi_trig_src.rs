#[doc = "Register `MSS_SPI_TRIG_SRC` reader"]
pub type R = crate::R<MssSpiTrigSrcSpec>;
#[doc = "Register `MSS_SPI_TRIG_SRC` writer"]
pub type W = crate::W<MssSpiTrigSrcSpec>;
#[doc = "Field `trig_spia` reader - 1:0\\]
RESERVED:Dont Touch"]
pub type TrigSpiaR = crate::FieldReader;
#[doc = "Field `trig_spia` writer - 1:0\\]
RESERVED:Dont Touch"]
pub type TrigSpiaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `trig_spib` reader - 26:16\\]
Writing 1'b1 to each bit will trigger MSS_SPIB Trigger&lt;0-10> respectively"]
pub type TrigSpibR = crate::FieldReader<u16>;
#[doc = "Field `trig_spib` writer - 26:16\\]
Writing 1'b1 to each bit will trigger MSS_SPIB Trigger&lt;0-10> respectively"]
pub type TrigSpibW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVED:Dont Touch"]
    #[inline(always)]
    pub fn trig_spia(&self) -> TrigSpiaR {
        TrigSpiaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Writing 1'b1 to each bit will trigger MSS_SPIB Trigger&lt;0-10> respectively"]
    #[inline(always)]
    pub fn trig_spib(&self) -> TrigSpibR {
        TrigSpibR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
RESERVED:Dont Touch"]
    #[inline(always)]
    #[must_use]
    pub fn trig_spia(&mut self) -> TrigSpiaW<MssSpiTrigSrcSpec> {
        TrigSpiaW::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Writing 1'b1 to each bit will trigger MSS_SPIB Trigger&lt;0-10> respectively"]
    #[inline(always)]
    #[must_use]
    pub fn trig_spib(&mut self) -> TrigSpibW<MssSpiTrigSrcSpec> {
        TrigSpibW::new(self, 16)
    }
}
#[doc = "MSS_SPI_TRIG_SRC\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_spi_trig_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_spi_trig_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssSpiTrigSrcSpec;
impl crate::RegisterSpec for MssSpiTrigSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_spi_trig_src::R`](R) reader structure"]
impl crate::Readable for MssSpiTrigSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_spi_trig_src::W`](W) writer structure"]
impl crate::Writable for MssSpiTrigSrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_SPI_TRIG_SRC to value 0"]
impl crate::Resettable for MssSpiTrigSrcSpec {
    const RESET_VALUE: u32 = 0;
}
