#[doc = "Register `CH45_HIL_CP_OVERRIDE` reader"]
pub type R = crate::R<Ch45HilCpOverrideSpec>;
#[doc = "Register `CH45_HIL_CP_OVERRIDE` writer"]
pub type W = crate::W<Ch45HilCpOverrideSpec>;
#[doc = "Field `chirp4` reader - 15:0\\]
Override data used for Chirp4. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp4R = crate::FieldReader<u16>;
#[doc = "Field `chirp4` writer - 15:0\\]
Override data used for Chirp4. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `chirp5` reader - 31:16\\]
Override data used for Chirp5. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp5R = crate::FieldReader<u16>;
#[doc = "Field `chirp5` writer - 31:16\\]
Override data used for Chirp5. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp4. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp4(&self) -> Chirp4R {
        Chirp4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp5. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp5(&self) -> Chirp5R {
        Chirp5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp4. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp4(&mut self) -> Chirp4W<Ch45HilCpOverrideSpec> {
        Chirp4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp5. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp5(&mut self) -> Chirp5W<Ch45HilCpOverrideSpec> {
        Chirp5W::new(self, 16)
    }
}
#[doc = "CH45_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch45_hil_cp_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch45_hil_cp_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch45HilCpOverrideSpec;
impl crate::RegisterSpec for Ch45HilCpOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch45_hil_cp_override::R`](R) reader structure"]
impl crate::Readable for Ch45HilCpOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ch45_hil_cp_override::W`](W) writer structure"]
impl crate::Writable for Ch45HilCpOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH45_HIL_CP_OVERRIDE to value 0"]
impl crate::Resettable for Ch45HilCpOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
