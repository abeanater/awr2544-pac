#[doc = "Register `MDIO_POLL_EN_REG` reader"]
pub type R = crate::R<MdioPollEnRegSpec>;
#[doc = "Register `MDIO_POLL_EN_REG` writer"]
pub type W = crate::W<MdioPollEnRegSpec>;
#[doc = "Field `MDIO_POLL_ENABLE` reader - 31:0\\]
MDIO Poll Enable"]
pub type MdioPollEnableR = crate::FieldReader<u32>;
#[doc = "Field `MDIO_POLL_ENABLE` writer - 31:0\\]
MDIO Poll Enable"]
pub type MdioPollEnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Poll Enable"]
    #[inline(always)]
    pub fn mdio_poll_enable(&self) -> MdioPollEnableR {
        MdioPollEnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO Poll Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_poll_enable(&mut self) -> MdioPollEnableW<MdioPollEnRegSpec> {
        MdioPollEnableW::new(self, 0)
    }
}
#[doc = "MDIO Poll Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdio_poll_en_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdio_poll_en_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdioPollEnRegSpec;
impl crate::RegisterSpec for MdioPollEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdio_poll_en_reg::R`](R) reader structure"]
impl crate::Readable for MdioPollEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`mdio_poll_en_reg::W`](W) writer structure"]
impl crate::Writable for MdioPollEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIO_POLL_EN_REG to value 0"]
impl crate::Resettable for MdioPollEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
