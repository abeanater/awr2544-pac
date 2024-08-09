#[doc = "Register `REGISTER7` reader"]
pub type R = crate::R<Register7Spec>;
#[doc = "Register `REGISTER7` writer"]
pub type W = crate::W<Register7Spec>;
#[doc = "Field `EMPTY` reader - 1:0\\]
Reserved"]
pub type EmptyR = crate::FieldReader;
#[doc = "Field `EMPTY` writer - 1:0\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGHSTXCOREEN` reader - 6:2\\]
1: Enable 0: Disable(default)"]
pub type ReghstxcoreenR = crate::FieldReader;
#[doc = "Field `REGHSTXCOREEN` writer - 6:2\\]
1: Enable 0: Disable(default)"]
pub type ReghstxcoreenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDHSTXCOREEN` reader - 7:7\\]
1: Override with register 0: Default"]
pub type OvrrdhstxcoreenR = crate::BitReader;
#[doc = "Field `OVRRDHSTXCOREEN` writer - 7:7\\]
1: Override with register 0: Default"]
pub type OvrrdhstxcoreenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBIASGENTRIMMODE` reader - 8:8\\]
1: Enable 0: Disable"]
pub type RegbiasgentrimmodeR = crate::BitReader;
#[doc = "Field `REGBIASGENTRIMMODE` writer - 8:8\\]
1: Enable 0: Disable"]
pub type RegbiasgentrimmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDBIASGENTRIMMODE` reader - 9:9\\]
1: Override with register 0: Default"]
pub type OvrrdbiasgentrimmodeR = crate::BitReader;
#[doc = "Field `OVRRDBIASGENTRIMMODE` writer - 9:9\\]
1: Override with register 0: Default"]
pub type OvrrdbiasgentrimmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHSLDOOBSERVE` reader - 10:10\\]
1: Enable HS LDO observe 0: Disable"]
pub type ReghsldoobserveR = crate::BitReader;
#[doc = "Field `REGHSLDOOBSERVE` writer - 10:10\\]
1: Enable HS LDO observe 0: Disable"]
pub type ReghsldoobserveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDHSLDOOBSERVE` reader - 11:11\\]
1: Override with register 0: Default"]
pub type OvrrdhsldoobserveR = crate::BitReader;
#[doc = "Field `OVRRDHSLDOOBSERVE` writer - 11:11\\]
1: Override with register 0: Default"]
pub type OvrrdhsldoobserveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLDOVDDTRACKING` reader - 12:12\\]
1: Enable LDO tracking VDD 0: Disable(default)"]
pub type RegldovddtrackingR = crate::BitReader;
#[doc = "Field `REGLDOVDDTRACKING` writer - 12:12\\]
1: Enable LDO tracking VDD 0: Disable(default)"]
pub type RegldovddtrackingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRDLDOVDDTRACKING` reader - 13:13\\]
1: Override with register 0: Default"]
pub type OvrrdldovddtrackingR = crate::BitReader;
#[doc = "Field `OVRRDLDOVDDTRACKING` writer - 13:13\\]
1: Override with register 0: Default"]
pub type OvrrdldovddtrackingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGULPRXEN` reader - 18:14\\]
1:Enable 0:Disable"]
pub type RegulprxenR = crate::FieldReader;
#[doc = "Field `REGULPRXEN` writer - 18:14\\]
1:Enable 0:Disable"]
pub type RegulprxenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDULPRXEN` reader - 19:19\\]
1: Override with register 0: Default"]
pub type OvrrdulprxenR = crate::BitReader;
#[doc = "Field `OVRRDULPRXEN` writer - 19:19\\]
1: Override with register 0: Default"]
pub type OvrrdulprxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLPCDEN` reader - 24:20\\]
1:Enable 0:Disable"]
pub type ReglpcdenR = crate::FieldReader;
#[doc = "Field `REGLPCDEN` writer - 24:20\\]
1:Enable 0:Disable"]
pub type ReglpcdenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDLPCDEN` reader - 25:25\\]
1: Override with register 0: Default"]
pub type OvrrdlpcdenR = crate::BitReader;
#[doc = "Field `OVRRDLPCDEN` writer - 25:25\\]
1: Override with register 0: Default"]
pub type OvrrdlpcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGLPRXEN` reader - 30:26\\]
1:Enable 0:Disable"]
pub type ReglprxenR = crate::FieldReader;
#[doc = "Field `REGLPRXEN` writer - 30:26\\]
1:Enable 0:Disable"]
pub type ReglprxenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OVRRDLPRXEN` reader - 31:31\\]
1: Override with register 0: Default"]
pub type OvrrdlprxenR = crate::BitReader;
#[doc = "Field `OVRRDLPRXEN` writer - 31:31\\]
1: Override with register 0: Default"]
pub type OvrrdlprxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - 6:2\\]
1: Enable 0: Disable(default)"]
    #[inline(always)]
    pub fn reghstxcoreen(&self) -> ReghstxcoreenR {
        ReghstxcoreenR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdhstxcoreen(&self) -> OvrrdhstxcoreenR {
        OvrrdhstxcoreenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Enable 0: Disable"]
    #[inline(always)]
    pub fn regbiasgentrimmode(&self) -> RegbiasgentrimmodeR {
        RegbiasgentrimmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdbiasgentrimmode(&self) -> OvrrdbiasgentrimmodeR {
        OvrrdbiasgentrimmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Enable HS LDO observe 0: Disable"]
    #[inline(always)]
    pub fn reghsldoobserve(&self) -> ReghsldoobserveR {
        ReghsldoobserveR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdhsldoobserve(&self) -> OvrrdhsldoobserveR {
        OvrrdhsldoobserveR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable LDO tracking VDD 0: Disable(default)"]
    #[inline(always)]
    pub fn regldovddtracking(&self) -> RegldovddtrackingR {
        RegldovddtrackingR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdldovddtracking(&self) -> OvrrdldovddtrackingR {
        OvrrdldovddtrackingR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:18 - 18:14\\]
1:Enable 0:Disable"]
    #[inline(always)]
    pub fn regulprxen(&self) -> RegulprxenR {
        RegulprxenR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdulprxen(&self) -> OvrrdulprxenR {
        OvrrdulprxenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24 - 24:20\\]
1:Enable 0:Disable"]
    #[inline(always)]
    pub fn reglpcden(&self) -> ReglpcdenR {
        ReglpcdenR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdlpcden(&self) -> OvrrdlpcdenR {
        OvrrdlpcdenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1:Enable 0:Disable"]
    #[inline(always)]
    pub fn reglprxen(&self) -> ReglprxenR {
        ReglprxenR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register 0: Default"]
    #[inline(always)]
    pub fn ovrrdlprxen(&self) -> OvrrdlprxenR {
        OvrrdlprxenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register7Spec> {
        EmptyW::new(self, 0)
    }
    #[doc = "Bits 2:6 - 6:2\\]
1: Enable 0: Disable(default)"]
    #[inline(always)]
    #[must_use]
    pub fn reghstxcoreen(&mut self) -> ReghstxcoreenW<Register7Spec> {
        ReghstxcoreenW::new(self, 2)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhstxcoreen(&mut self) -> OvrrdhstxcoreenW<Register7Spec> {
        OvrrdhstxcoreenW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
1: Enable 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regbiasgentrimmode(&mut self) -> RegbiasgentrimmodeW<Register7Spec> {
        RegbiasgentrimmodeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdbiasgentrimmode(&mut self) -> OvrrdbiasgentrimmodeW<Register7Spec> {
        OvrrdbiasgentrimmodeW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
1: Enable HS LDO observe 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reghsldoobserve(&mut self) -> ReghsldoobserveW<Register7Spec> {
        ReghsldoobserveW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdhsldoobserve(&mut self) -> OvrrdhsldoobserveW<Register7Spec> {
        OvrrdhsldoobserveW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
1: Enable LDO tracking VDD 0: Disable(default)"]
    #[inline(always)]
    #[must_use]
    pub fn regldovddtracking(&mut self) -> RegldovddtrackingW<Register7Spec> {
        RegldovddtrackingW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdldovddtracking(&mut self) -> OvrrdldovddtrackingW<Register7Spec> {
        OvrrdldovddtrackingW::new(self, 13)
    }
    #[doc = "Bits 14:18 - 18:14\\]
1:Enable 0:Disable"]
    #[inline(always)]
    #[must_use]
    pub fn regulprxen(&mut self) -> RegulprxenW<Register7Spec> {
        RegulprxenW::new(self, 14)
    }
    #[doc = "Bit 19 - 19:19\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdulprxen(&mut self) -> OvrrdulprxenW<Register7Spec> {
        OvrrdulprxenW::new(self, 19)
    }
    #[doc = "Bits 20:24 - 24:20\\]
1:Enable 0:Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reglpcden(&mut self) -> ReglpcdenW<Register7Spec> {
        ReglpcdenW::new(self, 20)
    }
    #[doc = "Bit 25 - 25:25\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdlpcden(&mut self) -> OvrrdlpcdenW<Register7Spec> {
        OvrrdlpcdenW::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
1:Enable 0:Disable"]
    #[inline(always)]
    #[must_use]
    pub fn reglprxen(&mut self) -> ReglprxenW<Register7Spec> {
        ReglprxenW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
1: Override with register 0: Default"]
    #[inline(always)]
    #[must_use]
    pub fn ovrrdlprxen(&mut self) -> OvrrdlprxenW<Register7Spec> {
        OvrrdlprxenW::new(self, 31)
    }
}
#[doc = "REGISTER7\n\nYou can [`read`](crate::Reg::read) this register and get [`register7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register7Spec;
impl crate::RegisterSpec for Register7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register7::R`](R) reader structure"]
impl crate::Readable for Register7Spec {}
#[doc = "`write(|w| ..)` method takes [`register7::W`](W) writer structure"]
impl crate::Writable for Register7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER7 to value 0"]
impl crate::Resettable for Register7Spec {
    const RESET_VALUE: u32 = 0;
}
