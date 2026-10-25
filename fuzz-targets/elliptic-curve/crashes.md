# elliptic-curve - crashes

Source: https://github.com/RustCrypto/traits @ 08780da1ae8ddad136b29fe77a93731d40c7ef38

<table>
<tr><th>Code</th><th>Crash</th></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::
    Add&lt;elliptic_curve::dev::mock_curve::ProjectivePoint&gt;&gt;::add(
    ProjectivePoint::from(
    &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::generator()),
    ProjectivePoint::from(
    &lt;elliptic_curve::point::NonIdentity&lt;elliptic_curve::dev::mock_curve::
    ProjectivePoint&gt; as ::core::convert::TryFrom&lt;elliptic_curve::dev::
    mock_curve::ProjectivePoint&gt;&gt;::try_from(
    &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    Group&gt;::generator()).unwrap()));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:823:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Add&gt;::add (elliptic-curve/src/dev/mock_curve.rs:823:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_823_9_e5814a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::try_from(Scalar::default()).unwrap();
let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::
    AffineCoordinates&gt;::x(&amp;AffinePoint::FixedBaseOutput(scalar));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:449:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates&gt;::x (elliptic-curve/src/dev/mock_curve.rs:449:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_449_9_e12fe7::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let projective_point = &lt;elliptic_curve::dev::mock_curve::
    ProjectivePoint as elliptic_curve::Group&gt;::identity();
let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::
    Sub&lt;elliptic_curve::dev::mock_curve::ProjectivePoint&gt;&gt;::sub(
    ProjectivePoint::Identity, projective_point);</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:851:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Sub&gt;::sub (elliptic-curve/src/dev/mock_curve.rs:851:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_851_9_de0550::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let scalar = Scalar::try_from(Scalar::from(
    &lt;elliptic_curve::ScalarValue&lt;elliptic_curve::dev::mock_curve::MockCurve&gt;&gt;::
    ZERO)).unwrap();
let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::
    Mul&lt;elliptic_curve::dev::mock_curve::Scalar&gt;&gt;::mul(
    AffinePoint::FixedBaseOutput(scalar), scalar);</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:570:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as core::ops::arith::Mul&lt;elliptic_curve::dev::mock_curve::Scalar&gt;&gt;::mul (elliptic-curve/src/dev/mock_curve.rs:570:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_570_9_aad232::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::Scalar as ::core::ops::
    Mul&lt;elliptic_curve::dev::mock_curve::Scalar&gt;&gt;::mul(Scalar::default(),
    Scalar::default());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:259:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as core::ops::arith::Mul&gt;::mul (elliptic-curve/src/dev/mock_curve.rs:259:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_259_9_b1ad4b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field&gt;::
    invert(&amp;Scalar::default());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:119:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as ff::Field&gt;::invert (elliptic-curve/src/dev/mock_curve.rs:119:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_119_9_7f56e7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::
    AffineCoordinates&gt;::y(
    &amp;&lt;elliptic_curve::point::NonIdentity&lt;elliptic_curve::dev::mock_curve::
    AffinePoint&gt; as ::core::convert::TryFrom&lt;elliptic_curve::dev::mock_curve::
    AffinePoint&gt;&gt;::try_from(AffinePoint::Generator).unwrap().to_point());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:453:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates&gt;::y (elliptic-curve/src/dev/mock_curve.rs:453:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_453_9_aca93e::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::
    Sub&lt;elliptic_curve::dev::mock_curve::AffinePoint&gt;&gt;::sub(
    &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    Group&gt;::generator(), AffinePoint::default());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:919:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Sub&lt;elliptic_curve::dev::mock_curve::AffinePoint&gt;&gt;::sub (elliptic-curve/src/dev/mock_curve.rs:919:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_919_9_c9702b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::to_curve(
    &amp;&lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::generator());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:482:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as group::CurveAffine&gt;::to_curve (elliptic-curve/src/dev/mock_curve.rs:482:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_482_9_83ebcb::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::is_identity(
    &amp;&lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::identity());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:478:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as group::CurveAffine&gt;::is_identity (elliptic-curve/src/dev/mock_curve.rs:478:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_478_9_f10b4d::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::
    Mul&lt;elliptic_curve::dev::mock_curve::Scalar&gt;&gt;::mul(
    &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    Group&gt;::identity(), Scalar::default());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:949:18:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Mul&lt;elliptic_curve::dev::mock_curve::Scalar&gt;&gt;::mul (elliptic-curve/src/dev/mock_curve.rs:949:18)
  repro_elliptic_curve_src_dev_mock_curve_rs_949_18_cf418a::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    Group&gt;::double(&amp;ProjectivePoint::FixedBaseOutput(Scalar::from(
    &lt;elliptic_curve::ScalarValue&lt;elliptic_curve::dev::mock_curve::
    MockCurve&gt; as ::core::default::Default&gt;::default())));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:756:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as group::Group&gt;::double (elliptic-curve/src/dev/mock_curve.rs:756:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_756_9_97b31c::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let scalar_value = &lt;elliptic_curve::ScalarValue&lt;elliptic_curve::dev::
    mock_curve::MockCurve&gt; as ::core::convert::From&lt;u64&gt;&gt;::from(1u64);
let string = scalar_value.to_string();
let _ = &lt;elliptic_curve::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::
    MockCurve&gt; as elliptic_curve::ops::Invert&gt;::invert_vartime(
    &amp;&lt;elliptic_curve::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::
    MockCurve&gt; as ::core::str::FromStr&gt;::from_str(&amp;string).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:325:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as crypto_bigint::traits::Invert&gt;::invert (elliptic-curve/src/dev/mock_curve.rs:325:9)
  &lt;elliptic_curve::dev::mock_curve::Scalar as crypto_bigint::traits::Invert&gt;::invert_vartime (crypto-bigint/src/traits.rs:984:14)
  &lt;elliptic_curve::scalar::nonzero::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::MockCurve&gt; as crypto_bigint::traits::Invert&gt;::invert_vartime (elliptic-curve/src/scalar/nonzero.rs:229:21)
  repro_elliptic_curve_src_dev_mock_curve_rs_325_9_30c813::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let projective_point = ProjectivePoint::Generator;
let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    CurveGroup&gt;::to_affine(&amp;projective_point);</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:811:18:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as group::Curve&gt;::to_affine (elliptic-curve/src/dev/mock_curve.rs:811:18)
  repro_elliptic_curve_src_dev_mock_curve_rs_811_18_23e014::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field&gt;::
    sqrt(&amp;Scalar::from(1u64));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:123:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as ff::Field&gt;::sqrt (elliptic-curve/src/dev/mock_curve.rs:123:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_123_9_da66d9::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field&gt;::
    square(&amp;Scalar::from(0u64));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:111:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as ff::Field&gt;::square (elliptic-curve/src/dev/mock_curve.rs:111:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_111_9_952d8b::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::Neg&gt;::neg(
    &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::
    CurveAffine&gt;::identity());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:606:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as core::ops::arith::Neg&gt;::neg (elliptic-curve/src/dev/mock_curve.rs:606:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_606_9_b06125::main (src/main.rs:2:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::Scalar as elliptic_curve::Field&gt;::
    sqrt_ratio(&amp;Scalar::default(),
    &amp;Scalar::from(
    &lt;elliptic_curve::ScalarValue&lt;elliptic_curve::dev::mock_curve::MockCurve&gt;&gt;::
    ZERO));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:127:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::Scalar as ff::Field&gt;::sqrt_ratio (elliptic-curve/src/dev/mock_curve.rs:127:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_127_9_d1ee9c::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let scalar_value = &lt;elliptic_curve::ScalarValue&lt;elliptic_curve::dev::
    mock_curve::MockCurve&gt;&gt;::ONE;
let string = scalar_value.to_string();
let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as ::core::ops::
    Mul&lt;elliptic_curve::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::
    MockCurve&gt;&gt;&gt;::mul(AffinePoint::Generator,
    &lt;elliptic_curve::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::
    MockCurve&gt; as ::core::str::FromStr&gt;::from_str(&amp;string).unwrap());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:598:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as core::ops::arith::Mul&lt;elliptic_curve::scalar::nonzero::NonZeroScalar&lt;elliptic_curve::dev::mock_curve::MockCurve&gt;&gt;&gt;::mul (elliptic-curve/src/dev/mock_curve.rs:598:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_598_9_919dc5::main (src/main.rs:6:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::
    AffineCoordinates&gt;::x_is_odd(&amp;AffinePoint::FixedBaseOutput(
    Scalar::default()));</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:457:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates&gt;::x_is_odd (elliptic-curve/src/dev/mock_curve.rs:457:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_457_9_24962c::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::
    AffineCoordinates&gt;::y_is_odd(&amp;AffinePoint::default());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:461:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::AffinePoint as elliptic_curve::point::AffineCoordinates&gt;::y_is_odd (elliptic-curve/src/dev/mock_curve.rs:461:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_461_9_797ed7::main (src/main.rs:4:13)</pre></td></tr>
<tr><td valign="top"><pre>let affine_point = &lt;elliptic_curve::dev::mock_curve::
    AffinePoint as elliptic_curve::CurveAffine&gt;::generator();
let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::
    Add&lt;elliptic_curve::dev::mock_curve::AffinePoint&gt;&gt;::add(
    ProjectivePoint::default(), affine_point);</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:879:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Add&lt;elliptic_curve::dev::mock_curve::AffinePoint&gt;&gt;::add (elliptic-curve/src/dev/mock_curve.rs:879:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_879_9_09f1c7::main (src/main.rs:5:13)</pre></td></tr>
<tr><td valign="top"><pre>let _ = &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as ::core::ops::Neg&gt;::
    neg(&lt;elliptic_curve::dev::mock_curve::ProjectivePoint as elliptic_curve::
    Group&gt;::generator());</pre></td><td valign="top"><pre>thread 'main' panicked at elliptic-curve/src/dev/mock_curve.rs:1000:9:
not implemented
backtrace:
  &lt;elliptic_curve::dev::mock_curve::ProjectivePoint as core::ops::arith::Neg&gt;::neg (elliptic-curve/src/dev/mock_curve.rs:1000:9)
  repro_elliptic_curve_src_dev_mock_curve_rs_1000_9_775601::main (src/main.rs:2:13)</pre></td></tr>
</table>
