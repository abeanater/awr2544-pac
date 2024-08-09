#[doc = "Register `TPTC_DBS_CONFIG` reader"]
pub type R = crate::R<TptcDbsConfigSpec>;
#[doc = "Register `TPTC_DBS_CONFIG` writer"]
pub type W = crate::W<TptcDbsConfigSpec>;
#[doc = "Field `tptc_a0` reader - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
pub type TptcA0R = crate::FieldReader;
#[doc = "Field `tptc_a0` writer - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
pub type TptcA0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_a1` reader - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
pub type TptcA1R = crate::FieldReader;
#[doc = "Field `tptc_a1` writer - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
pub type TptcA1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_b0` reader - 5:4\\]
RESERVED: Dont Use"]
pub type TptcB0R = crate::FieldReader;
#[doc = "Field `tptc_b0` writer - 5:4\\]
RESERVED: Dont Use"]
pub type TptcB0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_b1` reader - 7:6\\]
RESERVED: Dont Use"]
pub type TptcB1R = crate::FieldReader;
#[doc = "Field `tptc_b1` writer - 7:6\\]
RESERVED: Dont Use"]
pub type TptcB1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c0` reader - 9:8\\]
RESERVED: Dont Use0"]
pub type TptcC0R = crate::FieldReader;
#[doc = "Field `tptc_c0` writer - 9:8\\]
RESERVED: Dont Use0"]
pub type TptcC0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c1` reader - 11:10\\]
RESERVED: Dont Use"]
pub type TptcC1R = crate::FieldReader;
#[doc = "Field `tptc_c1` writer - 11:10\\]
RESERVED: Dont Use"]
pub type TptcC1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c2` reader - 13:12\\]
RESERVED: Dont Use"]
pub type TptcC2R = crate::FieldReader;
#[doc = "Field `tptc_c2` writer - 13:12\\]
RESERVED: Dont Use"]
pub type TptcC2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c3` reader - 15:14\\]
RESERVED: Dont Use"]
pub type TptcC3R = crate::FieldReader;
#[doc = "Field `tptc_c3` writer - 15:14\\]
RESERVED: Dont Use"]
pub type TptcC3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c4` reader - 17:16\\]
RESERVED: Dont Use"]
pub type TptcC4R = crate::FieldReader;
#[doc = "Field `tptc_c4` writer - 17:16\\]
RESERVED: Dont Use"]
pub type TptcC4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_c5` reader - 19:18\\]
RESERVED: Dont Use"]
pub type TptcC5R = crate::FieldReader;
#[doc = "Field `tptc_c5` writer - 19:18\\]
RESERVED: Dont Use"]
pub type TptcC5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
    #[inline(always)]
    pub fn tptc_a1(&self) -> TptcA1R {
        TptcA1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b0(&self) -> TptcB0R {
        TptcB0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b1(&self) -> TptcB1R {
        TptcB1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
RESERVED: Dont Use0"]
    #[inline(always)]
    pub fn tptc_c0(&self) -> TptcC0R {
        TptcC0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c1(&self) -> TptcC1R {
        TptcC1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c2(&self) -> TptcC2R {
        TptcC2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c3(&self) -> TptcC3R {
        TptcC3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c4(&self) -> TptcC4R {
        TptcC4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 19:18\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c5(&self) -> TptcC5R {
        TptcC5R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Max Burst size tieoff value for TPTC A0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<TptcDbsConfigSpec> {
        TptcA0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Max Burst size tieoff value for TPTC A1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1(&mut self) -> TptcA1W<TptcDbsConfigSpec> {
        TptcA1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0(&mut self) -> TptcB0W<TptcDbsConfigSpec> {
        TptcB0W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1(&mut self) -> TptcB1W<TptcDbsConfigSpec> {
        TptcB1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
RESERVED: Dont Use0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c0(&mut self) -> TptcC0W<TptcDbsConfigSpec> {
        TptcC0W::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c1(&mut self) -> TptcC1W<TptcDbsConfigSpec> {
        TptcC1W::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c2(&mut self) -> TptcC2W<TptcDbsConfigSpec> {
        TptcC2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c3(&mut self) -> TptcC3W<TptcDbsConfigSpec> {
        TptcC3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c4(&mut self) -> TptcC4W<TptcDbsConfigSpec> {
        TptcC4W::new(self, 16)
    }
    #[doc = "Bits 18:19 - 19:18\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c5(&mut self) -> TptcC5W<TptcDbsConfigSpec> {
        TptcC5W::new(self, 18)
    }
}
#[doc = "TPTC_DBS_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TptcDbsConfigSpec;
impl crate::RegisterSpec for TptcDbsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tptc_dbs_config::R`](R) reader structure"]
impl crate::Readable for TptcDbsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`tptc_dbs_config::W`](W) writer structure"]
impl crate::Writable for TptcDbsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPTC_DBS_CONFIG to value 0"]
impl crate::Resettable for TptcDbsConfigSpec {
    const RESET_VALUE: u32 = 0;
}
