#[doc = "Register `MDIO_POLL_REG` reader"]
pub type R = crate::R<MdioPollRegSpec>;
#[doc = "Register `MDIO_POLL_REG` writer"]
pub type W = crate::W<MdioPollRegSpec>;
#[doc = "Field `MDIO_IPG` reader - 7:0\\]
MDIO IPG"]
pub type MdioIpgR = crate::FieldReader;
#[doc = "Field `MDIO_IPG` writer - 7:0\\]
MDIO IPG"]
pub type MdioIpgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MDIO_STATE_CHANGE` reader - 30:30\\]
MDIO State Change Mode"]
pub type MdioStateChangeR = crate::BitReader;
#[doc = "Field `MDIO_STATE_CHANGE` writer - 30:30\\]
MDIO State Change Mode"]
pub type MdioStateChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_MANUAL_MODE` reader - 31:31\\]
MDIO Manual Mode"]
pub type MdioManualModeR = crate::BitReader;
#[doc = "Field `MDIO_MANUAL_MODE` writer - 31:31\\]
MDIO Manual Mode"]
pub type MdioManualModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
MDIO IPG"]
    #[inline(always)]
    pub fn mdio_ipg(&self) -> MdioIpgR {
        MdioIpgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
MDIO State Change Mode"]
    #[inline(always)]
    pub fn mdio_state_change(&self) -> MdioStateChangeR {
        MdioStateChangeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO Manual Mode"]
    #[inline(always)]
    pub fn mdio_manual_mode(&self) -> MdioManualModeR {
        MdioManualModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
MDIO IPG"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_ipg(&mut self) -> MdioIpgW<MdioPollRegSpec> {
        MdioIpgW::new(self, 0)
    }
    #[doc = "Bit 30 - 30:30\\]
MDIO State Change Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_state_change(&mut self) -> MdioStateChangeW<MdioPollRegSpec> {
        MdioStateChangeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
MDIO Manual Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_manual_mode(&mut self) -> MdioManualModeW<MdioPollRegSpec> {
        MdioManualModeW::new(self, 31)
    }
}
#[doc = "MDIO Poll Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_poll_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_poll_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioPollRegSpec;
impl crate::RegisterSpec for MdioPollRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_poll_reg::R`](R) reader structure"]
impl crate::Readable for MdioPollRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_poll_reg::W`](W) writer structure"]
impl crate::Writable for MdioPollRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_POLL_REG to value 0"]
impl crate::Resettable for MdioPollRegSpec {
    const RESET_VALUE: u32 = 0;
}
