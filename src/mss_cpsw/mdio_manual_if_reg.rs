#[doc = "Register `MDIO_MANUAL_IF_REG` reader"]
pub type R = crate::R<MdioManualIfRegSpec>;
#[doc = "Register `MDIO_MANUAL_IF_REG` writer"]
pub type W = crate::W<MdioManualIfRegSpec>;
#[doc = "Field `MDIO_PIN` reader - 0:0\\]
MDIO Pin"]
pub type MdioPinR = crate::BitReader;
#[doc = "Field `MDIO_PIN` writer - 0:0\\]
MDIO Pin"]
pub type MdioPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_OUTPUT_ENABLE` reader - 1:1\\]
MDIO Output Enable"]
pub type MdioOutputEnableR = crate::BitReader;
#[doc = "Field `MDIO_OUTPUT_ENABLE` writer - 1:1\\]
MDIO Output Enable"]
pub type MdioOutputEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_CLOCK_OUTPUT` reader - 2:2\\]
MDIO Clock Output"]
pub type MdioClockOutputR = crate::BitReader;
#[doc = "Field `MDIO_CLOCK_OUTPUT` writer - 2:2\\]
MDIO Clock Output"]
pub type MdioClockOutputW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MDIO Pin"]
    #[inline(always)]
    pub fn mdio_pin(&self) -> MdioPinR {
        MdioPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MDIO Output Enable"]
    #[inline(always)]
    pub fn mdio_output_enable(&self) -> MdioOutputEnableR {
        MdioOutputEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MDIO Clock Output"]
    #[inline(always)]
    pub fn mdio_clock_output(&self) -> MdioClockOutputR {
        MdioClockOutputR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MDIO Pin"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_pin(&mut self) -> MdioPinW<MdioManualIfRegSpec> {
        MdioPinW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MDIO Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_output_enable(&mut self) -> MdioOutputEnableW<MdioManualIfRegSpec> {
        MdioOutputEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
MDIO Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_clock_output(&mut self) -> MdioClockOutputW<MdioManualIfRegSpec> {
        MdioClockOutputW::new(self, 2)
    }
}
#[doc = "MDIO Manual Interface Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_manual_if_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_manual_if_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioManualIfRegSpec;
impl crate::RegisterSpec for MdioManualIfRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_manual_if_reg::R`](R) reader structure"]
impl crate::Readable for MdioManualIfRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_manual_if_reg::W`](W) writer structure"]
impl crate::Writable for MdioManualIfRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_MANUAL_IF_REG to value 0"]
impl crate::Resettable for MdioManualIfRegSpec {
    const RESET_VALUE: u32 = 0;
}
