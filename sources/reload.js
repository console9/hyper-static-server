
(function () {
	
	"use strict";
	
	var _tokenOld = "";
	
	async function _reload () {
		
		var _tokenNew = "";
		
		try {
			
			let _response = await fetch ("/__/reload.txt");
			
			if (_response.status == 200) {
				_tokenNew = await _response.text ();
			}
			
		} catch (_error) {
			console.error (_error);
		}
		
		if ((_tokenNew != "") && (_tokenOld == "")) {
			_tokenOld = _tokenNew;
		} else if ((_tokenNew == "") && (_tokenOld != "")) {
			_tokenNew = _tokenOld;
		}
		
		if (_tokenOld != _tokenNew) {
			if (window.history.scrollRestoration !== undefined) {
				window.history.scrollRestoration = "auto";
			}
			window.history.go ();
			return;
		}
		
		window.setTimeout (_reload, 1000);
	}
	
	window.setTimeout (_reload, 1000);
	
}) ();

