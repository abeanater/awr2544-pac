#[doc = "Register `GIODIRF` reader"]
pub type R = crate::R<GiodirfSpec>;
#[doc = "Register `GIODIRF` writer"]
pub type W = crate::W<GiodirfSpec>;
#[doc = "Field `GIODIRF` reader - 7:0\\]
GIO data direction of pins in Port F"]
pub type GiodirfR = crate::FieldReader;
#[doc = "Field `GIODIRF` writer - 7:0\\]
GIO data direction of pins in Port F"]
pub type GiodirfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU10` reader - 31:8\\]
Reserved"]
pub type Nu10R = crate::FieldReader<u32>;
#[doc = "Field `NU10` writer - 31:8\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port F"]
    #[inline(always)]
    pub fn giodirf(&self) -> GiodirfR {
        GiodirfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data direction of pins in Port F"]
    #[inline(always)]
    #[must_use]
    pub fn giodirf(&mut self) -> GiodirfW<GiodirfSpec> {
        GiodirfW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<GiodirfSpec> {
        Nu10W::new(self, 8)
    }
}
#[doc = "GIO data direction of pins in Port F\n\nYou can [`read`](crate::Reg::read) this register and get [`giodirf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodirf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodirfSpec;
impl crate::RegisterSpec for GiodirfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodirf::R`](R) reader structure"]
impl crate::Readable for GiodirfSpec {}
#[doc = "`write(|w| ..)` method takes [`giodirf::W`](W) writer structure"]
impl crate::Writable for GiodirfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODIRF to value 0"]
impl crate::Resettable for GiodirfSpec {
    const RESET_VALUE: u32 = 0;
}
