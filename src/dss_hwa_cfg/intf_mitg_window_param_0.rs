#[doc = "Register `INTF_MITG_WINDOW_PARAM_0` reader"]
pub type R = crate::R<IntfMitgWindowParam0Spec>;
#[doc = "Register `INTF_MITG_WINDOW_PARAM_0` writer"]
pub type W = crate::W<IntfMitgWindowParam0Spec>;
#[doc = "Field `intf_mitg_window_param_0` reader - 4:0\\]
This is a programmable array of window parameters. Each window parameter is an unsigned 5 bit integer. The total length of the array is 5. The BFR of the array is given by the matlab code : val = round(hanning(12)*32) INTF_MITG_WINDOW_PARAM = val(1:5);"]
pub type IntfMitgWindowParam0R = crate::FieldReader;
#[doc = "Field `intf_mitg_window_param_0` writer - 4:0\\]
This is a programmable array of window parameters. Each window parameter is an unsigned 5 bit integer. The total length of the array is 5. The BFR of the array is given by the matlab code : val = round(hanning(12)*32) INTF_MITG_WINDOW_PARAM = val(1:5);"]
pub type IntfMitgWindowParam0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
This is a programmable array of window parameters. Each window parameter is an unsigned 5 bit integer. The total length of the array is 5. The BFR of the array is given by the matlab code : val = round(hanning(12)*32) INTF_MITG_WINDOW_PARAM = val(1:5);"]
    #[inline(always)]
    pub fn intf_mitg_window_param_0(&self) -> IntfMitgWindowParam0R {
        IntfMitgWindowParam0R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
This is a programmable array of window parameters. Each window parameter is an unsigned 5 bit integer. The total length of the array is 5. The BFR of the array is given by the matlab code : val = round(hanning(12)*32) INTF_MITG_WINDOW_PARAM = val(1:5);"]
    #[inline(always)]
    #[must_use]
    pub fn intf_mitg_window_param_0(&mut self) -> IntfMitgWindowParam0W<IntfMitgWindowParam0Spec> {
        IntfMitgWindowParam0W::new(self, 0)
    }
}
#[doc = "INTF_MITG_WINDOW_PARAM_0\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mitg_window_param_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mitg_window_param_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMitgWindowParam0Spec;
impl crate::RegisterSpec for IntfMitgWindowParam0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_mitg_window_param_0::R`](R) reader structure"]
impl crate::Readable for IntfMitgWindowParam0Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_mitg_window_param_0::W`](W) writer structure"]
impl crate::Writable for IntfMitgWindowParam0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MITG_WINDOW_PARAM_0 to value 0"]
impl crate::Resettable for IntfMitgWindowParam0Spec {
    const RESET_VALUE: u32 = 0;
}
