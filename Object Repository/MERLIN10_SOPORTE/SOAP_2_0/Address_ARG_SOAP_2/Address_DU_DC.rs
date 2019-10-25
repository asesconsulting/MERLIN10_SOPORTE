<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_DU_DC</name>
   <tag></tag>
   <elementGuidId>313ca1ce-8481-4641-a9fc-2f6c914d37b2</elementGuidId>
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
               &lt;level4>INGENIERO MASCHWITZ&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>Barrio MASCHWITZ 1387&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>1623&lt;/postalCode>
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


/*Verificacion TestCase**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DC')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '6')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ESCOBAR')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'INGENIERO MASCHWITZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'BARRIO MASCHWITZ 1387')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.383353')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.750027')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1623')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '1387')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '3')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>ESCOBAR&lt;/level3>&lt;level4>MAQUINISTA F SAVIO&lt;/level4>&lt;level5>MASCHWITZ PRIVADO&lt;/level5>&lt;street>BO MASCHWITZ PRIVADO&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-34.392926&lt;/latitude>&lt;longitude>-58.766695&lt;/longitude>&lt;postalCode>1619&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>ESCOBAR&lt;/level3>&lt;level4>INGENIERO MASCHWITZ&lt;/level4>&lt;level5>MASCHWITZ VILLAGE&lt;/level5>&lt;street>BO MASCHWITZ VILLAGE&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-34.373931&lt;/latitude>&lt;longitude>-58.745573&lt;/longitude>&lt;postalCode>1619&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1>&lt;/level1>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>ESCOBAR&lt;/level3>&lt;level4>INGENIERO MASCHWITZ&lt;/level4>&lt;level5>MASCHWITZ CLUB&lt;/level5>&lt;street>BO MASCHWITZ CLUB&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-34.377518&lt;/latitude>&lt;longitude>-58.754644&lt;/longitude>&lt;postalCode>1619&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.750027&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.383353&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.383353&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.750027&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nivel4Abreviada15&lt;/name>&lt;value>ING MASCHWITCHZ&lt;/value>')


/**verificacion WS Unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'DU')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'DC')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '6')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'ESCOBAR')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'INGENIERO MASCHWITZ')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'BARRIO MASCHWITZ 1387')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.383353')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.750027')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1623')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '0')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalData', '1387')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '3')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>ESCOBAR&lt;/level3>&lt;level4>INGENIERO MASCHWITZ&lt;/level4>&lt;level5>MASCHWITZ CLUB&lt;/level5>&lt;street>BO MASCHWITZ CLUB&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-34.377518&lt;/latitude>&lt;longitude>-58.754644&lt;/longitude>&lt;postalCode>1623&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;alternativeAddresses>&lt;geoType>8&lt;/geoType>&lt;level1/>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level3>ESCOBAR&lt;/level3>&lt;level4>INGENIERO MASCHWITZ&lt;/level4>&lt;level5>MASCHWITZ VILLAGE&lt;/level5>&lt;street>BO MASCHWITZ VILLAGE&lt;/street>&lt;houseNumber>0&lt;/houseNumber>&lt;latitude>-34.373931&lt;/latitude>&lt;longitude>-58.745573&lt;/longitude>&lt;postalCode>1623&lt;/postalCode>&lt;fromStreetNumber>0&lt;/fromStreetNumber>&lt;toStreetNumber>0&lt;/toStreetNumber>&lt;/alternativeAddresses>')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.750027&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.383353&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>NISE&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>latitudLocalidad&lt;/name>&lt;value>-34.383353&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>longitudLocalidad&lt;/name>&lt;value>-58.750027&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>2&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nivel4Abreviada15&lt;/name>&lt;value>ING MASCHWITCHZ&lt;/value>')
**/</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
