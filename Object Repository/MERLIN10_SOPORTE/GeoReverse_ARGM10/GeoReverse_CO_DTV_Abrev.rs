<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GeoReverse_CO_DTV_Abrev</name>
   <tag></tag>
   <elementGuidId>72c73144-67e7-4df7-a42d-9ee637712f61</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.geo.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:geoReverse>
         &lt;!--Optional:-->
         &lt;geoReverse>
            &lt;!--Optional:-->
            &lt;clientAccessCode>59078f94b7c6bb08bf072449be86c2e0&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xGeoReverse>
               &lt;!--Optional:-->
               &lt;level1>AR&lt;/level1>
               &lt;!--Optional:-->
               &lt;latitude>-34.620629&lt;/latitude>
               &lt;!--Optional:-->
               &lt;longitude>-58.373310&lt;/longitude>
            &lt;/xGeoReverse>
         &lt;/geoReverse>
      &lt;/soap:geoReverse>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>geoReverse</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.GeoReverse_ARG2</defaultValue>
      <description></description>
      <id>9951f527-dc5d-4278-be2b-4d45d3c1ab69</id>
      <masked>false</masked>
      <name>GeoReverse_ARG2</name>
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




WS.verifyElementText(response, 'geoReverseResponse.return.status', 'CO')
WS.verifyElementText(response, 'geoReverseResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.level1', 'AR')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.level2', 'CAPITAL FEDERAL')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.level3', 'CAP FED')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.level4', 'CABA')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.level5', '')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.street', 'HUMBERTO PRIMO')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.streetType', '')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.approximateStreetNumber', '535')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.postalCode', '1103')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.additionalPostalCode', '')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.initialStreetSegment', '501')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.endStreetSegment', '600')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.betweenStreet1', 'GRAL SIMON BOLIVAR')
WS.verifyElementText(response, 'geoReverseResponse.return.nGeoReverse.betweenStreet2', 'PERU')
assertThat(response.getResponseText()).contains('&lt;name>zonaFranca&lt;/name>&lt;value>N&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoGeo&lt;/name>&lt;value>1&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>7&lt;/value>')</verificationScript>
   <wsdlAddress>${GeoReverse_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
