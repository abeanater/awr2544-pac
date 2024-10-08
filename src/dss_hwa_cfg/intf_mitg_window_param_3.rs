#[doc = "Register `INTF_MITG_WINDOW_PARAM_3` reader"]
pub type R = crate::R<IntfMitgWindowParam3Spec>;
#[doc = "Register `INTF_MITG_WINDOW_PARAM_3` writer"]
pub type W = crate::W<IntfMitgWindowParam3Spec>;
#[doc = "Field `intf_mitg_window_param_3` reader - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
pub type IntfMitgWindowParam3R = crate::FieldReader;
#[doc = "Field `intf_mitg_window_param_3` writer - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
pub type IntfMitgWindowParam3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
    #[inline(always)]
    pub fn intf_mitg_window_param_3(&self) -> IntfMitgWindowParam3R {
        IntfMitgWindowParam3R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Refer description of INTF_MITG_WINDOW_PARAM_0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_mitg_window_param_3(&mut self) -> IntfMitgWindowParam3W<IntfMitgWindowParam3Spec> {
        IntfMitgWindowParam3W::new(self, 0)
    }
}
#[doc = "INTF_MITG_WINDOW_PARAM_3\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMitgWindowParam3Spec;
impl crate::RegisterSpec for IntfMitgWindowParam3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_mitg_window_param_3::R`](R) reader structure"]
impl crate::Readable for IntfMitgWindowParam3Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_mitg_window_param_3::W`](W) writer structure"]
impl crate::Writable for IntfMitgWindowParam3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MITG_WINDOW_PARAM_3 to value 0"]
impl crate::Resettable for IntfMitgWindowParam3Spec {
    const RESET_VALUE: u32 = 0;
}
