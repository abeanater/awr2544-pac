#[doc = "Register `CH67_HIL_CP_OVERRIDE` reader"]
pub type R = crate::R<Ch67HilCpOverrideSpec>;
#[doc = "Register `CH67_HIL_CP_OVERRIDE` writer"]
pub type W = crate::W<Ch67HilCpOverrideSpec>;
#[doc = "Field `chirp6` reader - 15:0\\]
Override data used for Chirp6. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp6R = crate::FieldReader<u16>;
#[doc = "Field `chirp6` writer - 15:0\\]
Override data used for Chirp6. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `chirp7` reader - 31:16\\]
Override data used for Chirp7. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp7R = crate::FieldReader<u16>;
#[doc = "Field `chirp7` writer - 31:16\\]
Override data used for Chirp7. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp6. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp6(&self) -> Chirp6R {
        Chirp6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp7. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp7(&self) -> Chirp7R {
        Chirp7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp6. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp6(&mut self) -> Chirp6W<Ch67HilCpOverrideSpec> {
        Chirp6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp7. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp7(&mut self) -> Chirp7W<Ch67HilCpOverrideSpec> {
        Chirp7W::new(self, 16)
    }
}
#[doc = "CH67_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch67_hil_cp_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch67_hil_cp_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch67HilCpOverrideSpec;
impl crate::RegisterSpec for Ch67HilCpOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch67_hil_cp_override::R`](R) reader structure"]
impl crate::Readable for Ch67HilCpOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ch67_hil_cp_override::W`](W) writer structure"]
impl crate::Writable for Ch67HilCpOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH67_HIL_CP_OVERRIDE to value 0"]
impl crate::Resettable for Ch67HilCpOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
