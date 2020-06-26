<!--
M.m.p (YYYY-MM-DD)
==================
Add a summary of this release.

**BREAKING CHANGES**:

* Some change which breaks API or ABI compatiblity with.


Feature enhancements:

* [Link to github PR]():
  A new feature.

Bug fixes:

* [Link to github PR]():
  A bugfix.
-->
0.1.8 (2020-06-26)
==================
* Allow creation of devices with random AppKey and DevEui

0.1.7 (2020-06-02)
==================
* Allow for deletion of devices on TTN backend upon import
* Warn that ABP devices are not tolerated
* Provide error for unauthorized API key

0.1.6 (2020-05-12)
==================
No user-facing changes. Backend updated to fit production Console API.


0.1.5 (2020-03-13)
==================
Initial public release. Features include:
* create and delete devices records, using (app_eui, app_key, dev_eui) or UUID
* list all device records from an organization
* create and delete labels by UUID
* create and delete device labels, by using (device_uuid, label_uuid)
* import devices from The Things Network (TTN) with optional labeling with app id


