<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DA</name>
   <tag></tag>
   <elementGuidId>59fe6ae1-140c-4a61-bb71-52fd3600e625</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>Villa Ballester&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>SAN JOSE DE FLORES 6001&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>1653&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>4d59abac-efa3-41b7-bd77-7640915b76d1</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


/*verificacion TestCase*/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'VILLA BALLESTER')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'SAN JOSE DE FLORES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '6001')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1653')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '6')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>VILLA GODOY CRUZ&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>6201&lt;/fromStreetNumber>&lt;toStreetNumber>6900&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>VILLA GENERAL JOSE TOMAS GUIDO&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>4701&lt;/fromStreetNumber>&lt;toStreetNumber>6200&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>VILLA GRANADEROS DE SAN MARTIN&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>1301&lt;/fromStreetNumber>&lt;toStreetNumber>1600&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>VILLA GENERAL EUGENIO NECOCHEA&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>1201&lt;/fromStreetNumber>&lt;toStreetNumber>1800&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>VILLA BALLESTER&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>2001&lt;/fromStreetNumber>&lt;toStreetNumber>2300&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>GENERAL SAN MARTIN&lt;/level3>&lt;level4>VILLA GENERAL ANTONIO JOSE DE SUCRE&lt;/level4>&lt;level5>&lt;/level5>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>1653&lt;/postalCode>&lt;fromStreetNumber>5801&lt;/fromStreetNumber>&lt;toStreetNumber>6199&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.557521&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.542114&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.542114&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.557521&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')

/**verificacion WS unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'VILLA BALLESTER')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'SAN JOSE DE FLORES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '6001')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1653')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '6')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>VILLA GODOY CRUZ&lt;/level4>&lt;level5/>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>6201&lt;/fromStreetNumber>&lt;toStreetNumber>6900&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>VILLA GENERAL JOSE TOMAS GUIDO&lt;/level4>&lt;level5/>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>4701&lt;/fromStreetNumber>&lt;toStreetNumber>6200&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>VILLA GRANADEROS DE SAN MARTIN&lt;/level4>&lt;level5/>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>1301&lt;/fromStreetNumber>&lt;toStreetNumber>1600&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>VILLA GENERAL EUGENIO NECOCHEA&lt;/level4>&lt;level5/>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>1201&lt;/fromStreetNumber>&lt;toStreetNumber>1800&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>VILLA BALLESTER&lt;/level4>&lt;level5/>&lt;street>SAN JOSE&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>2001&lt;/fromStreetNumber>&lt;toStreetNumber>2300&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>GENERAL SAN MARTIN&lt;/level3>&lt;level4>VILLA GENERAL ANTONIO JOSE DE SUCRE&lt;/level4>&lt;level5/>&lt;street>SAN JOSE DE FLORES&lt;/street>&lt;houseNumber>6001&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>1653&lt;/postalCode>&lt;fromStreetNumber>5801&lt;/fromStreetNumber>&lt;toStreetNumber>6199&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.557521&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.542114&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.542114&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.557521&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
