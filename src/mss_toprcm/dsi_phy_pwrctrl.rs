#[doc = "Register `DSI_PHY_PWRCTRL` reader"]
pub type R = crate::R<DsiPhyPwrctrlSpec>;
#[doc = "Register `DSI_PHY_PWRCTRL` writer"]
pub type W = crate::W<DsiPhyPwrctrlSpec>;
#[doc = "Field `PONIN` reader - 0:0\\]
ON/OFF control of the weak power switch digital. For functional mode it should be 1"]
pub type PoninR = crate::BitReader;
#[doc = "Field `PONIN` writer - 0:0\\]
ON/OFF control of the weak power switch digital. For functional mode it should be 1"]
pub type PoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGOODIN` reader - 1:1\\]
ON/OFF control of the strong power switch digital. For functional mode it should be 1"]
pub type PgoodinR = crate::BitReader;
#[doc = "Field `PGOODIN` writer - 1:1\\]
ON/OFF control of the strong power switch digital. For functional mode it should be 1"]
pub type PgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO` reader - 2:2\\]
Save/Restore control for Isolation of output pins For functional mode it should be 0"]
pub type IsoR = crate::BitReader;
#[doc = "Field `ISO` writer - 2:2\\]
Save/Restore control for Isolation of output pins For functional mode it should be 0"]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOSCAN` reader - 3:3\\]
Save/Restore control for Isolation of the Scanout pins. For functional mode it should be 0"]
pub type IsoscanR = crate::BitReader;
#[doc = "Field `ISOSCAN` writer - 3:3\\]
Save/Restore control for Isolation of the Scanout pins. For functional mode it should be 0"]
pub type IsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PONOUT` reader - 4:4\\]
TI Reserved Dont touch."]
pub type PonoutR = crate::BitReader;
#[doc = "Field `PONOUT` writer - 4:4\\]
TI Reserved Dont touch."]
pub type PonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGOODOUT` reader - 5:5\\]
TI Reserved Dont touch."]
pub type PgoodoutR = crate::BitReader;
#[doc = "Field `PGOODOUT` writer - 5:5\\]
TI Reserved Dont touch."]
pub type PgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ON/OFF control of the weak power switch digital. For functional mode it should be 1"]
    #[inline(always)]
    pub fn ponin(&self) -> PoninR {
        PoninR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ON/OFF control of the strong power switch digital. For functional mode it should be 1"]
    #[inline(always)]
    pub fn pgoodin(&self) -> PgoodinR {
        PgoodinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Save/Restore control for Isolation of output pins For functional mode it should be 0"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Save/Restore control for Isolation of the Scanout pins. For functional mode it should be 0"]
    #[inline(always)]
    pub fn isoscan(&self) -> IsoscanR {
        IsoscanR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Reserved Dont touch."]
    #[inline(always)]
    pub fn ponout(&self) -> PonoutR {
        PonoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Reserved Dont touch."]
    #[inline(always)]
    pub fn pgoodout(&self) -> PgoodoutR {
        PgoodoutR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ON/OFF control of the weak power switch digital. For functional mode it should be 1"]
    #[inline(always)]
    #[must_use]
    pub fn ponin(&mut self) -> PoninW<DsiPhyPwrctrlSpec> {
        PoninW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ON/OFF control of the strong power switch digital. For functional mode it should be 1"]
    #[inline(always)]
    #[must_use]
    pub fn pgoodin(&mut self) -> PgoodinW<DsiPhyPwrctrlSpec> {
        PgoodinW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Save/Restore control for Isolation of output pins For functional mode it should be 0"]
    #[inline(always)]
    #[must_use]
    pub fn iso(&mut self) -> IsoW<DsiPhyPwrctrlSpec> {
        IsoW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Save/Restore control for Isolation of the Scanout pins. For functional mode it should be 0"]
    #[inline(always)]
    #[must_use]
    pub fn isoscan(&mut self) -> IsoscanW<DsiPhyPwrctrlSpec> {
        IsoscanW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Reserved Dont touch."]
    #[inline(always)]
    #[must_use]
    pub fn ponout(&mut self) -> PonoutW<DsiPhyPwrctrlSpec> {
        PonoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Reserved Dont touch."]
    #[inline(always)]
    #[must_use]
    pub fn pgoodout(&mut self) -> PgoodoutW<DsiPhyPwrctrlSpec> {
        PgoodoutW::new(self, 5)
    }
}
#[doc = "DSI_PHY_PWRCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_phy_pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_phy_pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPhyPwrctrlSpec;
impl crate::RegisterSpec for DsiPhyPwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_phy_pwrctrl::R`](R) reader structure"]
impl crate::Readable for DsiPhyPwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_phy_pwrctrl::W`](W) writer structure"]
impl crate::Writable for DsiPhyPwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSI_PHY_PWRCTRL to value 0"]
impl crate::Resettable for DsiPhyPwrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
