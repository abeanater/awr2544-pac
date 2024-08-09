#[doc = "Register `INTF_MITG_WINDOW_PARAM_1` reader"]
pub type R = crate::R<IntfMitgWindowParam1Spec>;
#[doc = "Register `INTF_MITG_WINDOW_PARAM_1` writer"]
pub type W = crate::W<IntfMitgWindowParam1Spec>;
#[doc = "Field `intf_mitg_window_param_1` reader - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
pub type IntfMitgWindowParam1R = crate::FieldReader;
#[doc = "Field `intf_mitg_window_param_1` writer - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
pub type IntfMitgWindowParam1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
    #[inline(always)]
    pub fn intf_mitg_window_param_1(&self) -> IntfMitgWindowParam1R {
        IntfMitgWindowParam1R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_mitg_window_param_1(&mut self) -> IntfMitgWindowParam1W<IntfMitgWindowParam1Spec> {
        IntfMitgWindowParam1W::new(self, 0)
    }
}
#[doc = "INTF_MITG_WINDOW_PARAM_1\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMitgWindowParam1Spec;
impl crate::RegisterSpec for IntfMitgWindowParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_mitg_window_param_1::R`](R) reader structure"]
impl crate::Readable for IntfMitgWindowParam1Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_mitg_window_param_1::W`](W) writer structure"]
impl crate::Writable for IntfMitgWindowParam1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MITG_WINDOW_PARAM_1 to value 0"]
impl crate::Resettable for IntfMitgWindowParam1Spec {
    const RESET_VALUE: u32 = 0;
}
