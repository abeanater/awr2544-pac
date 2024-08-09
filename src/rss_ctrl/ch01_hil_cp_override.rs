#[doc = "Register `CH01_HIL_CP_OVERRIDE` reader"]
pub type R = crate::R<Ch01HilCpOverrideSpec>;
#[doc = "Register `CH01_HIL_CP_OVERRIDE` writer"]
pub type W = crate::W<Ch01HilCpOverrideSpec>;
#[doc = "Field `chirp0` reader - 15:0\\]
Override data used for Chirp0. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp0R = crate::FieldReader<u16>;
#[doc = "Field `chirp0` writer - 15:0\\]
Override data used for Chirp0. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `chirp1` reader - 31:16\\]
Override data used for Chirp1. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp1R = crate::FieldReader<u16>;
#[doc = "Field `chirp1` writer - 31:16\\]
Override data used for Chirp1. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type Chirp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp0. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp0(&self) -> Chirp0R {
        Chirp0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp1. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp1(&self) -> Chirp1R {
        Chirp1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp0. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp0(&mut self) -> Chirp0W<Ch01HilCpOverrideSpec> {
        Chirp0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Override data used for Chirp1. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp1(&mut self) -> Chirp1W<Ch01HilCpOverrideSpec> {
        Chirp1W::new(self, 16)
    }
}
#[doc = "CH01_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch01_hil_cp_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch01_hil_cp_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch01HilCpOverrideSpec;
impl crate::RegisterSpec for Ch01HilCpOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch01_hil_cp_override::R`](R) reader structure"]
impl crate::Readable for Ch01HilCpOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ch01_hil_cp_override::W`](W) writer structure"]
impl crate::Writable for Ch01HilCpOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH01_HIL_CP_OVERRIDE to value 0"]
impl crate::Resettable for Ch01HilCpOverrideSpec {
    const RESET_VALUE: u32 = 0;
}
