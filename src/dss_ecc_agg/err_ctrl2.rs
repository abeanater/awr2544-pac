#[doc = "Register `err_ctrl2` reader"]
pub type R = crate::R<ErrCtrl2Spec>;
#[doc = "Register `err_ctrl2` writer"]
pub type W = crate::W<ErrCtrl2Spec>;
#[doc = "Field `DATA_BIT_THAT_1` reader - 15:0\\]
Data bit that needs to be flipped when force_sec is set"]
pub type DataBitThat1R = crate::FieldReader<u16>;
#[doc = "Field `DATA_BIT_THAT_1` writer - 15:0\\]
Data bit that needs to be flipped when force_sec is set"]
pub type DataBitThat1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATA_BIT_THAT` reader - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced"]
pub type DataBitThatR = crate::FieldReader<u16>;
#[doc = "Field `DATA_BIT_THAT` writer - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced"]
pub type DataBitThatW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Data bit that needs to be flipped when force_sec is set"]
    #[inline(always)]
    pub fn data_bit_that_1(&self) -> DataBitThat1R {
        DataBitThat1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced"]
    #[inline(always)]
    pub fn data_bit_that(&self) -> DataBitThatR {
        DataBitThatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Data bit that needs to be flipped when force_sec is set"]
    #[inline(always)]
    #[must_use]
    pub fn data_bit_that_1(&mut self) -> DataBitThat1W<ErrCtrl2Spec> {
        DataBitThat1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced"]
    #[inline(always)]
    #[must_use]
    pub fn data_bit_that(&mut self) -> DataBitThatW<ErrCtrl2Spec> {
        DataBitThatW::new(self, 16)
    }
}
#[doc = "ECC Error Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrCtrl2Spec;
impl crate::RegisterSpec for ErrCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_ctrl2::R`](R) reader structure"]
impl crate::Readable for ErrCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`err_ctrl2::W`](W) writer structure"]
impl crate::Writable for ErrCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets err_ctrl2 to value 0"]
impl crate::Resettable for ErrCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
