<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DA_Ruta</name>
   <tag></tag>
   <elementGuidId>99e8329d-3fbf-4f02-83e9-34a507e5e82a</elementGuidId>
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
               &lt;level2>buenos aires&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>florencio varela&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>ruta provincial 36&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber>3000&lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>1888&lt;/postalCode>
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
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'FLORENCIO VARELA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'RUTA PROVINCIAL 36')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '3000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1888')
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
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>ZEBALLOS&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>21,11&lt;/fromStreetNumber>&lt;toStreetNumber>22,5&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>QUILMES OESTE&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>11,31&lt;/fromStreetNumber>&lt;toStreetNumber>16,36&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>INGENIERO ALLAN&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>25,63&lt;/fromStreetNumber>&lt;toStreetNumber>28,00&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>EZPELETA OESTE&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>16,36&lt;/fromStreetNumber>&lt;toStreetNumber>16,4&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>BOSQUES&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>22,51&lt;/fromStreetNumber>&lt;toStreetNumber>25,63&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>&lt;/level3>&lt;level4>FLORENCIO VARELA&lt;/level4>&lt;level5>&lt;/level5>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode>&lt;/postalCode>&lt;fromStreetNumber>16,41&lt;/fromStreetNumber>&lt;toStreetNumber>21,11&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.795591&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.270691&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.795591&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.270691&lt;/value>')
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
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'FLORENCIO VARELA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'RUTA PROVINCIAL 36')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '3000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '00.000000')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1888')
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
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>ZEBALLOS&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>21,11&lt;/fromStreetNumber>&lt;toStreetNumber>22,5&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>QUILMES OESTE&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>11,31&lt;/fromStreetNumber>&lt;toStreetNumber>16,36&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>INGENIERO ALLAN&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>25,63&lt;/fromStreetNumber>&lt;toStreetNumber>28,00&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>EZPELETA OESTE&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>16,36&lt;/fromStreetNumber>&lt;toStreetNumber>16,4&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>BOSQUES&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>22,51&lt;/fromStreetNumber>&lt;toStreetNumber>25,63&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3/>&lt;level4>FLORENCIO VARELA&lt;/level4>&lt;level5/>&lt;street>RUTA PROV 36&lt;/street>&lt;houseNumber>3000&lt;/houseNumber>&lt;latitude>00.000000&lt;/latitude>&lt;longitude>00.000000&lt;/longitude>&lt;postalCode/>&lt;fromStreetNumber>16,41&lt;/fromStreetNumber>&lt;toStreetNumber>21,11&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.795591&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.270691&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>cpa&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>puerta&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>0&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.795591&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.270691&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>zonaRiesgo&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>NO RELEVADO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>0&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
